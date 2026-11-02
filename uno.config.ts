import { defineConfig, presetUno, presetAttributify, presetIcons } from 'unocss'

export default defineConfig({
  presets: [
    presetUno({ dark: 'class' }),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      cdn: 'https://esm.sh/',
    }),
  ],
  shortcuts: {
    'btn': 'px-4 py-2 rounded-md font-medium transition-all duration-150 cursor-pointer inline-flex items-center justify-center gap-1.5',
    'btn-primary': 'btn bg-primary text-on-primary hover:bg-primary-hover active:scale-98',
    'btn-secondary': 'btn border border-primary text-primary hover:bg-surface-hover',
    'btn-ghost': 'btn text-muted-fg hover:bg-surface-hover',
    'btn-destructive': 'btn bg-destructive text-on-destructive hover:bg-destructive-hover',
    'btn-icon': 'w-9 h-9 rounded-md flex items-center justify-center text-muted-fg hover:bg-surface-hover transition-all duration-150 cursor-pointer',
    'card': 'rounded-lg p-4 transition-all duration-200',
    'input-base': 'h-9 px-3 rounded-md border border-border bg-background text-foreground outline-none transition-all duration-150',
  },
  theme: {
    colors: {
      primary: {
        DEFAULT: '#D97706',
        hover: '#B45309',
        active: '#92400E',
      },
      secondary: {
        DEFAULT: '#F59E0B',
        hover: '#D97706',
      },
      accent: {
        DEFAULT: '#EA580C',
        hover: '#C2410C',
      },
      background: {
        DEFAULT: '#FFFBEB',
        dark: '#1C1917',
      },
      surface: {
        DEFAULT: '#FFFFFF',
        hover: '#FEF3C7',
        dark: '#292524',
        'hover-dark': '#44403C',
      },
      foreground: {
        DEFAULT: '#1C1917',
        dark: '#E7E0CC',
      },
      muted: {
        DEFAULT: '#F5F0E1',
        bg: '#FEF3C7',
        fg: '#78716C',
        'fg-dark': '#A8A29E',
      },
      border: {
        DEFAULT: '#E7E0CC',
        focus: '#D97706',
        dark: '#44403C',
      },
      destructive: {
        DEFAULT: '#DC2626',
        hover: '#B91C1C',
      },
      success: '#16A34A',
      warning: '#CA8A04',
      ring: '#D97706',
      on: {
        primary: '#FFFFFF',
        secondary: '#1C1917',
        accent: '#FFFFFF',
        destructive: '#FFFFFF',
      },
      note: {
        yellow: '#FEF3C7',
        blue: '#DBEAFE',
        green: '#DCFCE7',
        pink: '#FCE7F3',
        purple: '#F3E8FF',
        white: '#FFFFFF',
        text: '#1C1917',
      },
    },
    fontFamily: {
      ui: ['Plus Jakarta Sans', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'sans-serif'],
      note: ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Microsoft YaHei', 'sans-serif'],
      mono: ['JetBrains Mono', 'Cascadia Code', 'Consolas', 'monospace'],
    },
    fontSize: {
      xs: ['11px', { 'line-height': '1.4' }],
      sm: ['12px', { 'line-height': '1.5' }],
      base: ['14px', { 'line-height': '1.5' }],
      lg: ['16px', { 'line-height': '1.5' }],
      xl: ['18px', { 'line-height': '1.4' }],
      '2xl': ['24px', { 'line-height': '1.3' }],
    },
    borderRadius: {
      sm: '4px',
      md: '6px',
      lg: '8px',
      xl: '12px',
    },
    boxShadow: {
      sm: '0 1px 2px rgba(0, 0, 0, 0.05)',
      md: '0 4px 12px rgba(0, 0, 0, 0.08)',
      lg: '0 8px 24px rgba(0, 0, 0, 0.12)',
      xl: '0 12px 32px rgba(0, 0, 0, 0.16)',
    },
  },
})
