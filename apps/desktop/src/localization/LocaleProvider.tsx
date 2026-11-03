import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from "react";
import { availableLocales, translate, type MessageKey } from "./catalog";

type LocaleContextValue = {
  locale: string;
  locales: typeof availableLocales;
  setLocale: (locale: string) => void;
  t: (key: MessageKey, values?: Readonly<Record<string, string | number>>) => string;
};

const LocaleContext = createContext<LocaleContextValue | null>(null);

export function LocaleProvider({ children }: { children: ReactNode }) {
  const [locale, setActiveLocale] = useState("es");
  const setLocale = useCallback((next: string) => {
    if (availableLocales.some((option) => option.code === next)) setActiveLocale(next);
  }, []);
  const t = useCallback((key: MessageKey, values?: Readonly<Record<string, string | number>>) => translate(locale, key, values), [locale]);

  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);

  const contextValue = useMemo(() => ({ locale, locales: availableLocales, setLocale, t }), [locale, setLocale, t]);

  return <LocaleContext.Provider value={contextValue}>{children}</LocaleContext.Provider>;
}

export function useLocale() {
  const context = useContext(LocaleContext);
  if (!context) throw new Error("LocaleProvider is required for localized UI.");
  return context;
}
