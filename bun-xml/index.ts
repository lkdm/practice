import { match, P, Pattern } from "ts-pattern";
import { validationOptions, X2jOptions, XMLParser } from "fast-xml-parser";
import { _ } from "lodash";

const xml = `<?xml version="1.0" encoding="UTF-8"?>
<note>
    <to>Tove</to>
    <from>Jani</from>
    <heading>Reminder</heading>
    <body>Don't forget me this weekend!</body>
</note>`;

/**
 * Configuration for [`fast-xml-parser`]
 *
 * See: [Documentation](https://github.com/NaturalIntelligence/fast-xml-parser/blob/HEAD/docs/v4/2.XMLparseOptions.md)
 */
export interface FastXmlParseOpts extends X2jOptions { }

/**
 * Custom options for our XML parser
 */
interface CustomParseOpts {
	/** Filter out empty strings **/
	filterEmptyStringValues?: boolean;
	/** Turn dicts with numbered keys into arrays **/
	filterTurnNumberedKeyDictsToArrays?: boolean;
}
/**
 * Options for XML parser
 */
export interface ParseOpts extends Partial<FastXmlParseOpts>, CustomParseOpts { }

/**
 * Result monad
 */
export type Result<T, E = unknown> = Readonly<Ok<T>> | Readonly<Err<E>>;

/**
 * Create an okay type
 */
const ok = <T>(value: T): { ok: true; value: T } => ({ ok: true, value });

/**
 * Create an Error type
 */
const err = <E>(error: E): { ok: false; error: E } => ({ ok: false, error });
type Err<E> = ReturnType<typeof err<E>>;
type Ok<T> = ReturnType<typeof ok<T>>;
const isOk = <T, E>(result: Result<T, E>): result is Ok<T> => result.ok;
const isErr = <T, E>(result: Result<T, E>): result is Err<E> => !result.ok;

/**
 * Parses XML into a JSON object
 *
 * - Does not perform any validation
 */
export const parseXml = (
	input: string,
	opts: ParseOpts,
): Result<Record<string, unknown>, string> => {
	// Provide sane defaults
	const defaultOpts: Partial<FastXmlParseOpts> = {
		ignoreAttributes: true,
		attributeNamePrefix: "@_",
		allowBooleanAttributes: false,
		ignoreDeclaration: true,
		commentPropName: "#comment",
		// # Security
		//
		// Following attacks are possible due to entity processing:
		// - Denial-of-Service Attacks
		// - Classic XXE
		// - Advanced XXE
		// - Server-Side Request Forgery (SSRF)
		// - XInclude
		// - XSLT
		//
		// Since FXP doesn't allow entities with & in the values, above attacks should not work.
		//
		// Source: [Documentation](https://github.com/NaturalIntelligence/fast-xml-parser/blob/ad17aa4b12e2c052b6f3ae8de16c33192caf83ce/docs/v4/5.Entities.md#attacks)
		processEntities: false,
	};
	const parser = new XMLParser({
		...defaultOpts,
		...opts,
	});

	try {
		let parsedValue = parser.parse(input);
		// TODO: Make immutable data struct
		parsedValue = opts.filterEmptyStringValues
			? filterEmptyStrings(parsedValue)
			: parsedValue;
		// TODO: Make numbered key dicts into arrays
		parsedValue = opts.filterTurnNumberedKeyDictsToArrays
			? morphNumberedDictsToArrays(parsedValue)
			: parsedValue;
		return ok(parsedValue);
	} catch (error) {
		const message = error instanceof Error ? error.message : "unknown reason";
		return err(`could not parse XML because: ${message}`);
	}
};

const morphNumberedDictsToArrays = (
	obj: Record<string, unknown>,
): Record<string, unknown> => {
	// Step 1 - Detect an object being used in lieu of an array
	// Step 2 - Morph it into an array
	return obj;
};

const filterEmptyStrings = (
	obj: Record<string, unknown>,
): Record<string, unknown> => {
	return Object.entries(obj).reduce(
		(acc, [key, value]) => {
			if (value === "") {
				return acc; // Skip empty strings
			}
			if (typeof value === "object" && value !== null) {
				acc[key] = filterEmptyStrings(value as Record<string, unknown>); // Recursively filter
			} else {
				acc[key] = value; // Keep non-empty values
			}
			return acc;
		},
		{} as Record<string, unknown>,
	);
};

const main = async () => {
	const file = Bun.file("/var/home/luke/Downloads/test.xml");
	const text = await file.text();
	const out = parseXml(text, {});
	match(out)
		.with({ ok: true }, (out) =>
			console.log(JSON.stringify(filterEmptyStrings(out.value))),
		)
		.with({ ok: false }, ({ error }) => console.error(error))
		.exhaustive();
};

await main();
