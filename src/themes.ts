export const themes : { [theme: string]: { [key: string]: string } } = {
  'Fairy': {
    'primary-color': 'hsl(336, 89%, 93%)',
    'primary-color-shadow': 'hsl(336, 47%, 62%)',
    'secondary-color': 'hsl(336, 100%, 70%)',
    'tertiary-color': 'hsla(276, 57%, 67%, 1.00)',
    'background-color': 'hsla(307, 47%, 18%, 1.00)',
    'background-color-dark': 'hsl(307, 47%, 15%)',
    'text-color': 'white',
    'text-outline': 'hsl(276, 100%, 25%)'
  },
  'Coffee': {
    'primary-color': 'hsl(39, 59%, 78%)',
    'primary-color-shadow': 'hsl(39, 59%, 58%)',
    'secondary-color': 'hsl(18, 71%, 27%)',
    'tertiary-color': 'hsl(31, 51%, 34%)',
    'background-color': 'hsl(26, 42%, 19%)',
    'background-color-dark': 'hsl(26, 42%, 15%)',
    'text-color': 'hsl(0, 0%, 100%)',
    'text-outline': 'hsl(26, 42%, 19%)',
  },
  'Forest': {
    'primary-color': 'hsl(60, 63%, 89%)',
    'primary-color-shadow': 'hsl(60, 63%, 69%)',
    'secondary-color': 'hsl(77, 14%, 45%)',
    'tertiary-color': 'hsl(31, 43%, 50%)',
    'background-color': 'hsl(227, 8%, 22%)',
    'background-color-dark': 'hsl(227, 8%, 18%)',
    'text-color': 'hsl(0, 0%, 100%)',
    'text-outline': 'hsl(26, 62%, 18%)'
  }
};

export const setTheme = (theme: string) => {
  if (!Object.keys(themes).includes(theme)) return;
  const vars = themes[theme];
  const root = document.documentElement;

  Object.entries(vars).forEach(([key, value]) => {
    root.style.setProperty(`--${key}`, value);
  });
}
