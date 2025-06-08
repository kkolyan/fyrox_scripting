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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="scripting_api.html"><strong aria-hidden="true">1.</strong> Scripting API (C#)</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Input.html"><strong aria-hidden="true">1.1.</strong> Input</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Input/Input.html"><strong aria-hidden="true">1.1.1.</strong> Input</a></li><li class="chapter-item expanded "><a href="scripting_api/Input/KeyCode.html"><strong aria-hidden="true">1.1.2.</strong> KeyCode</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Log.html"><strong aria-hidden="true">1.2.</strong> Log</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Log/Log.html"><strong aria-hidden="true">1.2.1.</strong> Log</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Math.html"><strong aria-hidden="true">1.3.</strong> Math</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Math/Basis.html"><strong aria-hidden="true">1.3.1.</strong> Basis</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/EulerOrder.html"><strong aria-hidden="true">1.3.2.</strong> EulerOrder</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Mathf.html"><strong aria-hidden="true">1.3.3.</strong> Mathf</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Quaternion.html"><strong aria-hidden="true">1.3.4.</strong> Quaternion</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Vector2.html"><strong aria-hidden="true">1.3.5.</strong> Vector2</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Vector2I.html"><strong aria-hidden="true">1.3.6.</strong> Vector2I</a></li><li class="chapter-item expanded "><a href="scripting_api/Math/Vector3.html"><strong aria-hidden="true">1.3.7.</strong> Vector3</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Node.html"><strong aria-hidden="true">1.4.</strong> Node</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Node/Node.html"><strong aria-hidden="true">1.4.1.</strong> Node</a></li><li class="chapter-item expanded "><a href="scripting_api/Node/RoutingStrategy.html"><strong aria-hidden="true">1.4.2.</strong> RoutingStrategy</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Physics.html"><strong aria-hidden="true">1.5.</strong> Physics</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Physics/FeatureId.html"><strong aria-hidden="true">1.5.1.</strong> FeatureId</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/FeatureKind.html"><strong aria-hidden="true">1.5.2.</strong> FeatureKind</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/InteractionGroups.html"><strong aria-hidden="true">1.5.3.</strong> InteractionGroups</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/Intersection.html"><strong aria-hidden="true">1.5.4.</strong> Intersection</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/Physics.html"><strong aria-hidden="true">1.5.5.</strong> Physics</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/RayCastOptions.html"><strong aria-hidden="true">1.5.6.</strong> RayCastOptions</a></li><li class="chapter-item expanded "><a href="scripting_api/Physics/RigidBody.html"><strong aria-hidden="true">1.5.7.</strong> RigidBody</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Plugin.html"><strong aria-hidden="true">1.6.</strong> Plugin</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Plugin/GlobalScript.html"><strong aria-hidden="true">1.6.1.</strong> GlobalScript</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Prefab.html"><strong aria-hidden="true">1.7.</strong> Prefab</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Prefab/Prefab.html"><strong aria-hidden="true">1.7.1.</strong> Prefab</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Scene.html"><strong aria-hidden="true">1.8.</strong> Scene</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Scene/Scene.html"><strong aria-hidden="true">1.8.1.</strong> Scene</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/UI.html"><strong aria-hidden="true">1.9.</strong> UI</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/UI/Brush.html"><strong aria-hidden="true">1.9.1.</strong> Brush</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/Color.html"><strong aria-hidden="true">1.9.2.</strong> Color</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/GradientPoint.html"><strong aria-hidden="true">1.9.3.</strong> GradientPoint</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/LinearGradient.html"><strong aria-hidden="true">1.9.4.</strong> LinearGradient</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/RadialGradient.html"><strong aria-hidden="true">1.9.5.</strong> RadialGradient</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/Text.html"><strong aria-hidden="true">1.9.6.</strong> Text</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/TextBuilder.html"><strong aria-hidden="true">1.9.7.</strong> TextBuilder</a></li><li class="chapter-item expanded "><a href="scripting_api/UI/UiNode.html"><strong aria-hidden="true">1.9.8.</strong> UiNode</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api/Window.html"><strong aria-hidden="true">1.10.</strong> Window</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api/Window/CursorGrabMode.html"><strong aria-hidden="true">1.10.1.</strong> CursorGrabMode</a></li><li class="chapter-item expanded "><a href="scripting_api/Window/Window.html"><strong aria-hidden="true">1.10.2.</strong> Window</a></li></ol></li></ol></li></ol>';
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
