const LIGHT_MODE_BUTTON = String.raw`<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-sun" width="30" height="30" viewBox="0 0 30 30" stroke-width="2" stroke="#fff" fill="none" stroke-linecap="round" stroke-linejoin="round">
  <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
  <circle cx="12" cy="12" r="4" />
  <path d="M3 12h1m8 -9v1m8 8h1m-9 8v1m-6.4 -15.4l.7 .7m12.1 -.7l-.7 .7m0 11.4l.7 .7m-12.1 -.7l-.7 .7" />
</svg>`

const DARK_MODE_BUTTON = String.raw`<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-moon" width="30" height="30" viewBox="0 0 30 30" stroke-width="2" stroke="#000" fill="none" stroke-linecap="round" stroke-linejoin="round">
  <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
  <path d="M12 3c.132 0 .263 0 .393 0a7.5 7.5 0 0 0 7.92 12.446a9 9 0 1 1 -8.313 -12.454z" />
</svg>`

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
        button.innerHTML =
            next === 'dark' ? LIGHT_MODE_BUTTON : DARK_MODE_BUTTON
    }
}
