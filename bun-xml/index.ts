import { match } from "ts-pattern";
import { XMLParser } from "fast-xml-parser";

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
 * Does not perform any validation
 */
export const parseXml = (
	input: string,
	opts: Partial<ParseOpts>,
): Result<string, string> => {
	// const cleanedInput = input.replace(/<\?xml.*?\?>/, "").trim();
	// Set sane defaults
	const defaultOpts: ParseOpts = {
		ignoreAttributes: true,
		attributeNamePrefix: "@_",
		allowBooleanAttributes: false,
		ignoreDeclaration: true,
	};
	const parser = new XMLParser({ ...defaultOpts, ...opts });
	try {
		const parsedValue = parser.parse(input);
		return ok(parsedValue);
	} catch (error) {
		const message = error instanceof Error ? error.message : "unknown reason";
		return err(`could not parse XML because: ${message}`);
	}
};

// TODO: deal with `adjustmentList` empty string value
// In fact, all empty strings should just be null values?
//

// Medicare specific; certain numbers need to be strings

const main = async () => {
	const file = Bun.file("/var/home/luke/Downloads/test.xml");
	const text = await file.text();
	const out = parseXml(text, {});
	match(out)
		.with({ ok: true }, (out) => console.log(JSON.stringify(out.value)))
		.with({ ok: false }, ({ error }) => console.error(error))
		.exhaustive();
};

await main();
