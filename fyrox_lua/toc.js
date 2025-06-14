// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Getting started</a></li><li class="chapter-item expanded "><a href="scripting_api.html"><strong aria-hidden="true">2.</strong> Scripting API (Lua)</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Color.html"><strong aria-hidden="true">2.1.</strong> Color</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Color/Color.html"><strong aria-hidden="true">2.1.1.</strong> Color</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Input.html"><strong aria-hidden="true">2.2.</strong> Input</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Input/Input.html"><strong aria-hidden="true">2.2.1.</strong> Input</a></li><li class="chapter-item expanded "><a href="scripting_api/Input/KeyCode.html"><strong aria-hidden="true">2.2.2.</strong> KeyCode</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Log.html"><strong aria-hidden="true">2.3.</strong> Log</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Log/Log.html"><strong aria-hidden="true">2.3.1.</strong> Log</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Math.html"><strong aria-hidden="true">2.4.</strong> Math</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Math/Quaternion.html"><strong aria-hidden="true">2.4.1.</strong> Quaternion</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Vector2.html"><strong aria-hidden="true">2.4.2.</strong> Vector2</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Vector2I.html"><strong aria-hidden="true">2.4.3.</strong> Vector2I</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Vector3.html"><strong aria-hidden="true">2.4.4.</strong> Vector3</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Node.html"><strong aria-hidden="true">2.5.</strong> Node</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Node/Node.html"><strong aria-hidden="true">2.5.1.</strong> Node</a></li><li class="chapter-item expanded "><a href="scripting_api/Node/RoutingStrategy.html"><strong aria-hidden="true">2.5.2.</strong> RoutingStrategy</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Physics.html"><strong aria-hidden="true">2.6.</strong> Physics</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Physics/FeatureId.html"><strong aria-hidden="true">2.6.1.</strong> FeatureId</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/FeatureKind.html"><strong aria-hidden="true">2.6.2.</strong> FeatureKind</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/InteractionGroups.html"><strong aria-hidden="true">2.6.3.</strong> InteractionGroups</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/Intersection.html"><strong aria-hidden="true">2.6.4.</strong> Intersection</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/Physics.html"><strong aria-hidden="true">2.6.5.</strong> Physics</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/RayCastOptions.html"><strong aria-hidden="true">2.6.6.</strong> RayCastOptions</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/RigidBody.html"><strong aria-hidden="true">2.6.7.</strong> RigidBody</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Prefab.html"><strong aria-hidden="true">2.7.</strong> Prefab</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Prefab/Prefab.html"><strong aria-hidden="true">2.7.1.</strong> Prefab</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Scene.html"><strong aria-hidden="true">2.8.</strong> Scene</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Scene/Scene.html"><strong aria-hidden="true">2.8.1.</strong> Scene</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Script.html"><strong aria-hidden="true">2.9.</strong> Script</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Script/GlobalScript.html"><strong aria-hidden="true">2.9.1.</strong> GlobalScript</a></li><li class="chapter-item expanded "><a href="scripting_api/Script/NodeScript.html"><strong aria-hidden="true">2.9.2.</strong> NodeScript</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/UI.html"><strong aria-hidden="true">2.10.</strong> UI</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/UI/Brush.html"><strong aria-hidden="true">2.10.1.</strong> Brush</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/GradientPoint.html"><strong aria-hidden="true">2.10.2.</strong> GradientPoint</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/LinearGradient.html"><strong aria-hidden="true">2.10.3.</strong> LinearGradient</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/RadialGradient.html"><strong aria-hidden="true">2.10.4.</strong> RadialGradient</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/Text.html"><strong aria-hidden="true">2.10.5.</strong> Text</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/TextBuilder.html"><strong aria-hidden="true">2.10.6.</strong> TextBuilder</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/UiNode.html"><strong aria-hidden="true">2.10.7.</strong> UiNode</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Window.html"><strong aria-hidden="true">2.11.</strong> Window</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Window/CursorGrabMode.html"><strong aria-hidden="true">2.11.1.</strong> CursorGrabMode</a></li><li class="chapter-item expanded "><a href="scripting_api/Window/Window.html"><strong aria-hidden="true">2.11.2.</strong> Window</a></li></ol></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
