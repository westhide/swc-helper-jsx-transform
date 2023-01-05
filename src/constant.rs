use phf::{phf_set, Set};

/// ## [HTML ELEMENT](https://html.spec.whatwg.org/multipage/indices.html#elements-3)
pub const HTML_ELEMENT: &Set<&str> = &phf_set! {
    "a","abbr","address","area","article","aside","audio",
    "b","base","bdi","bdo","blockquote","body","br","button",
    "canvas","caption","cite","code","col","colgroup",
    "data","datalist","dd","del","details","dfn","dialog","div","dl","dt",
    "em","embed",
    "fieldset","figcaption","figure","footer","form",
    "h1","h2","h3","h4","h5","h6",
    "head","header","hgroup","hr","html",
    "i","iframe","img","input","ins",
    "kbd",
    "label","legend","li","link",
    "main","map","mark","menu","meta","meter",
    "nav","noscript",
    "object","ol","optgroup","option","output",
    "p","picture","pre","progress",
    "q",
    "rp","rt","ruby",
    "s","samp","script","section","select","slot","small","source",
    "span","strong","style","sub","summary","sup","svg",
    "table","tbody","td","template","textarea","tfoot",
    "th","thead","time","title","tr","track",
    "u","ul",
    "var","video",
    "wbr",
};

/// ## [SVG Element](https://svgwg.org/svg2-draft/eltindex.html)
pub const SVG_ELEMENT: &Set<&str> = &phf_set! {
    "a","animate","animateMotion","animateTransform","circle","clipPath",
    "defs","desc","discard","ellipse",
    "feBlend","feColorMatrix","feComponentTransfer","feComposite","feConvolveMatrix",
    "feDiffuseLighting","feDisplacementMap","feDistantLight","feDropShadow",
    "feFlood","feFuncA","feFuncB","feFuncG","feFuncR","feGaussianBlur",
    "feImage","feMerge","feMergeNode","feMorphology","feOffset","fePointLight",
    "feSpecularLighting","feSpotLight","feTile","feTurbulence","filter","foreignObject",
    "g","image","line","linearGradient",
    "marker","mask","metadata","mpath",
    "path","pattern","polygon","polyline",
    "radialGradient","rect",
    "script","set","stop","style","svg","switch","symbol",
    "text","textPath","title","tspan",
    "use","view",
};

/// ## [Boolean attributes](https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#boolean-attributes)
/// [Attributes](https://html.spec.whatwg.org/multipage/indices.html#attributes-3)
pub const BOOLEAN_ATTRIBUTE: &Set<&str> = &phf_set! {
    "allowfullscreen","async","autofocus","autoplay",
    "checked","controls",
    "default","defer","disabled",
    "formnovalidate","hidden",
    "inert","isMap","itemscope",
    "loop",
    "multiple","muted",
    "nomodule","novalidate",
    "open",
    "playsinline",
    "readonly","required","reversed",
    "selected",
};
