import { defineStore } from "pinia";
import { i18n, type AppLocale, DEFAULT_LOCALE, SUPPORTED_LOCALES } from "../../i18n";

const STORAGE_KEY = "app_locale";

const isSupportedLocale = (value: string): value is AppLocale => {
  return SUPPORTED_LOCALES.includes(value as AppLocale);
};

export const useLocaleStore = defineStore("localeStore", {
  state: () => ({
    locale: DEFAULT_LOCALE as AppLocale,
  }),
  actions: {
    init() {
      const savedLocale = localStorage.getItem(STORAGE_KEY);
      if (savedLocale && isSupportedLocale(savedLocale)) {
        this.locale = savedLocale;
      }
      i18n.global.locale.value = this.locale;
    },
    setLocale(nextLocale: AppLocale) {
      if (!isSupportedLocale(nextLocale)) {
        return;
      }
      this.locale = nextLocale;
      i18n.global.locale.value = nextLocale;
      localStorage.setItem(STORAGE_KEY, nextLocale);
    },
  },
});
