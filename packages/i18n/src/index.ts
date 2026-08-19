import enUS from '../locales/en-US.json';

export const locales = {
  'en-US': enUS,
};

export type LocaleKey = keyof typeof locales;
export type Messages = typeof enUS;
