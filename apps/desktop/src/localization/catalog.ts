import englishBase from "./locales/en.json";
import spanishBase from "./locales/es.json";

export type MessageKey = keyof typeof englishBase;
export type TranslationCatalog = Record<MessageKey, string>;

const baseCatalogs = {
  en: englishBase,
  es: validateCatalog(spanishBase),
} satisfies Record<string, TranslationCatalog>;

const localeFiles = import.meta.glob<Record<string, string>>("./locales/*.json", {
  eager: true,
  import: "default",
});

function toLocaleCode(path: string) {
  return path.slice(path.lastIndexOf("/") + 1, -5);
}

function validateCatalog(messages: Record<string, string>): TranslationCatalog {
  const baseKeys = new Set(Object.keys(englishBase));
  for (const key of Object.keys(englishBase)) {
    if (typeof messages[key] !== "string") {
      throw new Error("Every locale file must include the base translation keys.");
    }
  }
  if (Object.keys(messages).some((key) => !baseKeys.has(key))) {
    throw new Error("Locale files cannot define unknown translation keys.");
  }
  return messages as TranslationCatalog;
}

const additionalLocales = Object.entries(localeFiles)
  .filter(([path]) => !path.endsWith("/en.json") && !path.endsWith("/es.json"))
  .map(([path, messages]) => [toLocaleCode(path), validateCatalog(messages)]);

export const translationCatalogs = Object.fromEntries([
  ...Object.entries(baseCatalogs),
  ...additionalLocales,
]) as Record<string, TranslationCatalog>;

if (!translationCatalogs.es || !translationCatalogs.en) {
  throw new Error("The Spanish and English locale files are required.");
}

export const availableLocales = Object.entries(translationCatalogs)
  .map(([code, messages]) => ({ code, label: messages["locale.name"] }))
  .sort((left, right) => left.label.localeCompare(right.label));

export function translate(
  locale: string,
  key: MessageKey,
  values?: Readonly<Record<string, string | number>>,
) {
  let message = (translationCatalogs[locale] ?? translationCatalogs.es)[key] ?? englishBase[key];
  if (!values) return message;
  for (const [name, value] of Object.entries(values)) {
    message = message.replace(new RegExp(`\\{${name}\\}`, "g"), String(value));
  }
  return message;
}
