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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="scripting_api_cs.html"><strong aria-hidden="true">2.</strong> Scripting API for (C#)</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteInput.html"><strong aria-hidden="true">2.1.</strong> LiteInput</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteInput/Input.html"><strong aria-hidden="true">2.1.1.</strong> Input</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteInput/KeyCode.html"><strong aria-hidden="true">2.1.2.</strong> KeyCode</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteLog.html"><strong aria-hidden="true">2.2.</strong> LiteLog</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteLog/Log.html"><strong aria-hidden="true">2.2.1.</strong> Log</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath.html"><strong aria-hidden="true">2.3.</strong> LiteMath</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath/Basis.html"><strong aria-hidden="true">2.3.1.</strong> Basis</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath/EulerOrder.html"><strong aria-hidden="true">2.3.2.</strong> EulerOrder</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath/Mathf.html"><strong aria-hidden="true">2.3.3.</strong> Mathf</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath/Quaternion.html"><strong aria-hidden="true">2.3.4.</strong> Quaternion</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath/Vector2.html"><strong aria-hidden="true">2.3.5.</strong> Vector2</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath/Vector2I.html"><strong aria-hidden="true">2.3.6.</strong> Vector2I</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteMath/Vector3.html"><strong aria-hidden="true">2.3.7.</strong> Vector3</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteNode.html"><strong aria-hidden="true">2.4.</strong> LiteNode</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteNode/Node.html"><strong aria-hidden="true">2.4.1.</strong> Node</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteNode/RoutingStrategy.html"><strong aria-hidden="true">2.4.2.</strong> RoutingStrategy</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics.html"><strong aria-hidden="true">2.5.</strong> LitePhysics</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics/FeatureId.html"><strong aria-hidden="true">2.5.1.</strong> FeatureId</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics/FeatureKind.html"><strong aria-hidden="true">2.5.2.</strong> FeatureKind</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics/InteractionGroups.html"><strong aria-hidden="true">2.5.3.</strong> InteractionGroups</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics/Intersection.html"><strong aria-hidden="true">2.5.4.</strong> Intersection</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics/Physics.html"><strong aria-hidden="true">2.5.5.</strong> Physics</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics/RayCastOptions.html"><strong aria-hidden="true">2.5.6.</strong> RayCastOptions</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePhysics/RigidBody.html"><strong aria-hidden="true">2.5.7.</strong> RigidBody</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePlugin.html"><strong aria-hidden="true">2.6.</strong> LitePlugin</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LitePlugin/GlobalScript.html"><strong aria-hidden="true">2.6.1.</strong> GlobalScript</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LitePrefab.html"><strong aria-hidden="true">2.7.</strong> LitePrefab</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LitePrefab/Prefab.html"><strong aria-hidden="true">2.7.1.</strong> Prefab</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteScene.html"><strong aria-hidden="true">2.8.</strong> LiteScene</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteScene/Scene.html"><strong aria-hidden="true">2.8.1.</strong> Scene</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi.html"><strong aria-hidden="true">2.9.</strong> LiteUi</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/Brush.html"><strong aria-hidden="true">2.9.1.</strong> Brush</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/Color.html"><strong aria-hidden="true">2.9.2.</strong> Color</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/GradientPoint.html"><strong aria-hidden="true">2.9.3.</strong> GradientPoint</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/LinearGradient.html"><strong aria-hidden="true">2.9.4.</strong> LinearGradient</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/RadialGradient.html"><strong aria-hidden="true">2.9.5.</strong> RadialGradient</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/Text.html"><strong aria-hidden="true">2.9.6.</strong> Text</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/TextBuilder.html"><strong aria-hidden="true">2.9.7.</strong> TextBuilder</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteUi/UiNode.html"><strong aria-hidden="true">2.9.8.</strong> UiNode</a></li></ol></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteWindow.html"><strong aria-hidden="true">2.10.</strong> LiteWindow</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting_api_cs/LiteWindow/CursorGrabMode.html"><strong aria-hidden="true">2.10.1.</strong> CursorGrabMode</a></li><li class="chapter-item expanded "><a href="scripting_api_cs/LiteWindow/Window.html"><strong aria-hidden="true">2.10.2.</strong> Window</a></li></ol></li></ol></li></ol>';
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
