export function getStarsRankingUrl() {
  const users = [
    'antfu',
    'codecember',
    'knightly',
    'type-challenges',
    'vue-reactivity',
    'slidevjs',
  ]
  const repos = [
    'lokalise/i18n-ally',
    'vitejs/awesome-vite',
    'vitejs/vite',
    'vuejs/composition-api',
    'vueuse/vue-chemistry',
    'vueuse/vue-demi',
    'vueuse/vueuse',
    'wenyan-lang/ide',
    'wenyan-lang/wenyan',
    'wenyan-lang/wyg',
    'windicss/vite-plugin-windicss',
  ]

  const query = [
    ...users.map(i => `user:${i}`),
    ...repos.map(i => `repo:${i}`),
  ].join(' ')

  const url = `https://github.com/search?l=&o=desc&s=stars&type=Repositories&q=${encodeURIComponent(query)}`
  return url
}
