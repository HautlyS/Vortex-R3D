//! Book content data - pages, character info, abilities

pub struct Page {
    pub chapter: &'static str,
    pub title: &'static str,
    pub content: &'static str,
}

pub const PAGES: &[Page] = &[
    Page {
        chapter: "CHAPTER I",
        title: "The Digital Awakening",
        content: "In the beginning, there was the void—an infinite expanse of unrendered space. Then came the first vertex, a single point of light in the darkness, and from it, all geometry would flow.\n\nThe ancient programmers spoke of this moment in hushed tones, their fingers dancing across mechanical keyboards, invoking the sacred compile commands that would breathe life into silicon dreams.\n\n\"Let there be polygons,\" they whispered, and triangles tessellated across the void, forming the foundation of all virtual reality.",
    },
    Page {
        chapter: "CHAPTER II",
        title: "The Path of Pixels",
        content: "Each frame rendered is a meditation upon impermanence. Sixty times per second, the world dissolves and reforms, teaching us that nothing persists—only the illusion of continuity created by our limited perception.\n\nThe shader monks of the Eastern Rendering Temple spent decades perfecting their fragment programs, seeking the perfect balance between performance and beauty.\n\n\"Optimize not for speed alone,\" Master Carmack once taught, \"but for the harmony of all systems working as one.\"",
    },
    Page {
        chapter: "CHAPTER III",
        title: "Wisdom of the Wireframe",
        content: "Beneath every textured surface lies the wireframe truth. Strip away the normal maps, the ambient occlusion, the carefully crafted materials—and what remains? Pure geometry. Pure mathematics. Pure being.\n\nThe wireframe view is not a debug mode. It is enlightenment mode.\n\nWhen the student asked, \"Master, how do I achieve photorealism?\" the teacher replied, \"First, understand why you seek it.\"",
    },
    Page {
        chapter: "CHAPTER IV",
        title: "The Render Pipeline",
        content: "From vertex to fragment, the journey unfolds in stages both mysterious and precise. The GPU, that silicon bodhisattva, processes billions of operations each second, yet never complains, never wavers.\n\nThe Render Pipeline is the Eightfold Path:\n1. Input Assembly\n2. Vertex Shader\n3. Tessellation\n4. Geometry Shader\n5. Rasterization\n6. Fragment Shader\n7. Depth Testing\n8. Blending",
    },
    Page {
        chapter: "CHAPTER V",
        title: "Enlightenment Through Iteration",
        content: "The game loop is the wheel of dharma, turning endlessly:\n\nwhile (running) {\n    processInput();\n    update();\n    render();\n}\n\nIn these three functions lies all of existence. We receive input from the world, we update our internal state, we render our response.\n\nSeek not perfection, but stability. Seek not maximum performance, but sustainable performance. And always—profile before you optimize.",
    },
];

pub const CHARACTER_BIO: &str = "A wanderer between digital realms, the Seeker has traversed countless virtual landscapes in pursuit of the ultimate truth: the source code of consciousness itself.\n\nNow they walk the path of the Techno Sutra, gathering wisdom from ancient shader monks and modern compute prophets alike.";

pub const ABILITIES: &[(&str, &str, &str)] = &[
    (
        "🔮",
        "Digital Sight",
        "See through textures to the wireframe beneath",
    ),
    (
        "⚡",
        "Frame Skip",
        "Move between moments, bypassing time itself",
    ),
    ("🌀", "Shader Weave", "Manipulate light and shadow at will"),
    (
        "💫",
        "Buffer Overflow",
        "Channel excess data into raw power",
    ),
];
