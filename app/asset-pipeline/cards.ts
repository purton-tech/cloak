const clickableCards = document.querySelectorAll(".m_card.clickable");

clickableCards.forEach(function(card) {
    card.addEventListener('click', event => {
        event.preventDefault()
        console.log(event.target)

        if(card instanceof HTMLDivElement) {
            window.location.href = card.getAttribute('href')
        }
        return false
    })
});