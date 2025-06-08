
// document.querySelectorAll('a[href$="fake_home.html"]').forEach(a => {
//     a.href = "https://kkolyan.github.io/fyrox_lite/";
// });

const container = document.querySelector('ol.chapter');
if (container) {

    const li = document.createElement("li")
    li.className = "chapter-item"
    container.insertBefore(li, container.firstChild);

    const link = document.createElement('a');
    link.href = 'https://kkolyan.github.io/fyrox_lite/';
    link.textContent = 'Home⤴';
    li.insertBefore(link, null);
}
