document.querySelectorAll('select.remember').forEach((elem) => {
    if(elem instanceof HTMLSelectElement) {
        // Restore from local storage
        const selectedIndex = localStorage.getItem(elem.name)
        if(selectedIndex) {
            elem.selectedIndex = parseInt(selectedIndex)
        }
        // Add a click handler
        elem.addEventListener("change", () => {
            localStorage.setItem(elem.name, '' + elem.selectedIndex)
        })
    }
})
