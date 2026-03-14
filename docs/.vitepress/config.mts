import { defineConfig } from 'vitepress'

const base = '/'

export default defineConfig({
  title: 'KeyJey',
  description: 'Beautiful, Blazing-fast, Customizable Claude Code Statusline',
  base,

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: `${base}favicon.svg` }],
  ],

  themeConfig: {
    logo: '/logo.png',
    siteTitle: '⚓ KeyJey',

    nav: [
      { text: 'Guide', link: '/' },
      { text: 'Configuration', link: '/configuration' },
      { text: 'Passthrough', link: '/passthrough' },
      { text: 'Showcase', link: '/showcase' },
      { text: 'FAQ', link: '/faq' },
      {
        text: 'GitHub',
        link: 'https://github.com/KJ21-ENG/keyjey',
      },
    ],

    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'Configuration', link: '/configuration' },
        ],
      },
      {
        text: 'Advanced',
        items: [
          { text: 'Starship Passthrough', link: '/passthrough' },
        ],
      },
      {
        text: 'Community',
        items: [
          { text: 'Showcase', link: '/showcase' },
          { text: 'FAQ', link: '/faq' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/KJ21-ENG/keyjey' },
    ],

    footer: {
      message: 'Released under the Apache-2.0 License.',
      copyright: 'Copyright © KeyJey contributors',
    },

    search: {
      provider: 'local',
    },
  },
})
