import { match, P, Pattern } from "ts-pattern";
import { XMLParser } from "fast-xml-parser";
import { _ } from "lodash";

const xml = `<?xml version="1.0" encoding="UTF-8"?>
<note>
    <to>Tove</to>
    <from>Jani</from>
    <heading>Reminder</heading>
    <body>Don't forget me this weekend!</body>
</note>`;

/**
 * ParseOpts
 *
 * Configuration for [`fast-xml-parser`]
 *
 * See: [Documentation](https://github.com/NaturalIntelligence/fast-xml-parser/blob/HEAD/docs/v4/2.XMLparseOptions.md)
 */
interface ParseOpts {
	/** Whether to ignore the XML declaration **/
	ignoreDeclaration: boolean;
	/** Ignore attributes **/
	ignoreAttributes: boolean;
	/** Specify prefix of attributes **/
	attributeNamePrefix: string;
	/** Allow boolean attributes **/
	allowBooleanAttributes: boolean;
	/** Name to save XML comments into **/
	commentPropName: string | undefined;
}

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
	opts: Partial<ParseOpts>,
): Result<Record<string, unknown>, string> => {
	// Provide sane defaults
	const defaultOpts: ParseOpts = {
		ignoreAttributes: true,
		attributeNamePrefix: "@_",
		allowBooleanAttributes: false,
		ignoreDeclaration: true,
		commentPropName: "#comment",
	};
	const parser = new XMLParser({
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
		...defaultOpts,
		...opts,
	});
	try {
		const parsedValue = parser.parse(input);
		return ok(parsedValue);
	} catch (error) {
		const message = error instanceof Error ? error.message : "unknown reason";
		return err(`could not parse XML because: ${message}`);
	}
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
