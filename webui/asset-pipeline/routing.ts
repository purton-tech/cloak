import rlite from 'rlite-router';

const route = rlite(notFound, {
    // Default route
    '': function () {
        showSection('vaults')
    },

    'vault-details': function () {
        showSection('vault-details')
    },

    'audit': function () {
        showSection('audit')
    },

    // #inbox
    'users': function () {
        showSection('users')
    }
});

function notFound() {
    return '<h1>404 Not found :/</h1>';
  }

// Hash-based routing
function processHash() {
    const hash = location.hash || '#';

    // Do something useful with the result of the route
    route(hash.slice(1));
}

window.addEventListener('hashchange', processHash);
window.addEventListener('load', function () {
    processHash();
})

function showSection(name) {
    const sections = document.getElementsByTagName('section')
    for (var i = 0; i < sections.length; i++) {
        sections[i].style.display = 'none'
    }
    document.getElementById(name).style.display = null
    return false
}