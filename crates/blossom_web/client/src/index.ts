import '../static/main.css'
import 'src/theme.ts'

import { swap_icons } from 'src/theme'

const load = () => {
    // Check if the user has a previously saved theme preference and load that up.
    const preferredTheme =
        localStorage.getItem('theme') ||
        (window.matchMedia('(prefers-color-scheme: dark)').matches
            ? 'dark'
            : 'light')

    if (preferredTheme) {
        document.documentElement.setAttribute('data-theme', preferredTheme)
        swap_icons(preferredTheme)
    }
}

load()

export {}
