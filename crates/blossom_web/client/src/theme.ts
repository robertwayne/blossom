// Toggles between light and dark color themes.
const toggle = document.getElementById('toggle-btn')

if (toggle) {
    toggle.onclick = () => {
        const current = document.documentElement.getAttribute('data-theme')
        const target = current === 'dark' ? 'light' : 'dark'

        document.documentElement.setAttribute('data-theme', target)
        swap_icons(target)
        localStorage.setItem('theme', target)
    }
}

export const swap_icons = (next: string) => {
    const button = document.getElementById('toggle-btn')
    if (button) {
        button.innerText = next === 'dark' ? '☀️' : '🌙'
    }
}
