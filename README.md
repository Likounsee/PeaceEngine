# PeaceEngine

**PeaceEngine is an open-source, modular engine designed to power applications across the Likounsee ecosystem — from web browsing to emulation and graphical software.**

The goal is to build one reusable engine with a consistent foundation and public interfaces, rather than maintaining several unrelated rendering and runtime projects.

> **Project status: Early development.** PeaceEngine is being established. The architecture, APIs and supported features are not yet stable, and the capabilities described below are goals rather than claims about currently implemented functionality.

## Vision

PeaceEngine aims to provide a shared technology foundation that can be embedded into different applications:

- **Archia Browser** — use PeaceEngine as its browser and web-rendering engine.
- **PeaceEmu** — reuse suitable shared infrastructure such as graphics, input, resources and platform abstractions, while keeping console-specific emulation systems independent.
- **Likounsee Office Suite** — potentially use common text, image, graphics and composition facilities.
- **ArchiaOS** — potentially reuse platform-independent graphics and other suitable low-level components.
- **Third-party applications** — embed PeaceEngine through documented public APIs.

PeaceEngine is intended to be useful beyond the applications developed by Likounsee.

## One engine, modular internals

PeaceEngine is one project with one public identity and a coherent API. Its internal implementation can be divided into modules so applications only need the capabilities they use.

Planned areas include:

| Area | Responsibilities |
| --- | --- |
| Core | Lifecycle, memory management, resources, events and shared utilities |
| Graphics | Surfaces, 2D rendering, text, images and composition |
| Web | HTML, DOM, CSS, layout, JavaScript integration and web APIs |
| Platform | Windowing, input, display and operating-system integration |
| Networking | Network access and web resource loading |
| Embedding API | Public interfaces for integrating PeaceEngine into other applications |

These are architectural areas, not separate products or independent engines.

### Shared foundations, specialized systems

Not every application needs the same functionality. PeaceEngine will share general-purpose infrastructure where that makes sense, while keeping specialized systems independent.

For example, PeaceEmu's CPU, PPU, APU, cartridge and mapper implementations belong to its emulation layer. They should not be mixed into the web engine. The two projects may nevertheless benefit from common graphics, input, resource and platform abstractions.

## Design goals

- **Open source:** development should be transparent and contributions should be possible.
- **Embeddable:** applications should be able to integrate the engine without adopting Archia Browser's user interface.
- **Modular:** applications should not have to initialize every subsystem.
- **Portable:** keep platform-specific code behind clear interfaces.
- **Performance-conscious:** design for efficient memory use, rendering and resource management.
- **Reusable:** avoid duplicating general-purpose infrastructure across projects.
- **Maintainable:** favor clear boundaries, documented interfaces and automated tests.

## Relationship with Archia Browser

Archia Browser is intended to become the first major application powered by PeaceEngine.

The browser interface — tabs, address bar, menus, settings and browser-specific behavior — belongs to Archia Browser. PeaceEngine should provide the underlying engine capabilities through reusable interfaces.

This separation will allow other applications to embed the engine without depending on the browser's interface or application-specific code.

## Architecture principles

1. Keep a single PeaceEngine repository and a unified public API.
2. Separate reusable foundations from application-specific functionality.
3. Keep platform-dependent implementations behind abstractions.
4. Make optional capabilities independently usable where practical.
5. Test modules independently as well as through integration tests.
6. Avoid introducing dependencies on a particular application's UI framework.
7. Document public APIs and compatibility expectations as they stabilize.

## Development roadmap

The roadmap is intentionally incremental. Features will be marked complete only when implemented and tested.

- [ ] **Foundation** — repository structure, project conventions and core interfaces.
- [ ] **Core systems** — lifecycle, resources, events and memory foundations.
- [ ] **Graphics foundation** — software surfaces, drawing primitives, text and image handling.
- [ ] **Platform layer** — initial desktop platform integration and input handling.
- [ ] **Embedding API** — a documented interface for host applications.
- [ ] **Web foundation** — HTML parsing, DOM, CSS processing and layout.
- [ ] **JavaScript integration** — scripting runtime integration and selected Web APIs.
- [ ] **Archia Browser integration** — progressively move browser-engine responsibilities into PeaceEngine.
- [ ] **Additional integrations** — evaluate shared components for PeaceEmu, Likounsee Office Suite and ArchiaOS.

The order may evolve as implementation and testing reveal dependencies.

## Repository structure

The directory layout will be established as the implementation takes shape. The intended organization is modular, while keeping all engine components in this repository.

A possible direction is:

```text
PeaceEngine/
├── core/
├── graphics/
├── platform/
├── networking/
├── web/
├── embedding/
├── tests/
└── docs/
```

This is a proposed structure, not a description of files that already exist.

## Current limitations

PeaceEngine is at the beginning of development. At this stage:

- No stable embedding API is available yet.
- No complete browser engine is available yet.
- No complete JavaScript or modern web-platform implementation is claimed.
- No production-ready GPU backend is claimed.
- Compatibility with existing applications must be established through implementation and tests.

The roadmap describes intended work, not existing capabilities.

## Contributing

Contributions, technical discussions and architecture feedback are welcome.

Before implementing a large subsystem, please open an issue or discussion to agree on its scope and how it fits into PeaceEngine's shared architecture. Changes should include relevant tests and documentation.

## License

The project license has not yet been selected. A license will be added before the project establishes its contribution and redistribution terms.

---

**PeaceEngine — one engine, many applications.**
