document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        const clickableCards = document.querySelectorAll(".m_card.clickable");

        clickableCards.forEach(function (card) {
            card.addEventListener('click', event => {
                event.preventDefault()
                const href = card.getAttribute('href')

                if (card instanceof HTMLDivElement && href != null) {
                    window.location.href = href
                }
                return false
            })
        })
    }
})