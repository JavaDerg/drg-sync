function get_browser_preference(): boolean {
    return (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches)
            || !window.matchMedia
}

export function set_theme(theme: "light" | "dark") {
    switch (theme) {
        case "light":
            document.documentElement.classList.remove("dark");
            break;
        case "dark":
            document.documentElement.classList.add("dark");
            break;
    }

    if(localStorage.getItem("dark") !== null)
        localStorage.setItem("dark", theme);
}

export function get_theme(): "light" | "dark" {
    return document.documentElement.classList.contains("dark")
        ? "dark"
        : "light";
}

export function init() {
    if(localStorage === undefined) return;

    const val = localStorage.getItem("dark");
    switch (val) {
        case null:
            set_theme(
                get_browser_preference()
                    ? "dark"
                    : "light"
            );
            break;
        case "light":
        case "dark":
            set_theme(val);
            break;
    }
}
