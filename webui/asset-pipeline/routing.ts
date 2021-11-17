import rlite from 'rlite-router';

const route = rlite(notFound, {
    // Default route
    '': function () {
        showSection('vaults')
        document.getElementById('vaults-link').classList.add('selected')
    },

    'vault-details': function () {
        showSection('vault-details')
        document.getElementById('vaults-link').classList.add('selected')
    },

    'audit': function () {
        showSection('audit')
        document.getElementById('audit-link').classList.add('selected')
    },

    // #inbox
    'users': function () {
        showSection('users')
        document.getElementById('users-link').classList.add('selected')
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

    document.getElementById('vaults-link').classList.remove('selected')
    document.getElementById('users-link').classList.remove('selected')
    document.getElementById('audit-link').classList.remove('selected')
    return false
}