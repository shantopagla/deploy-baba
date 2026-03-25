-- Scala Computing
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Designed and developed multi-tenant backend APIs and distributed services in Go powering the core simulation platform', 'responsibility', 0),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Led consolidation of platform services into a Rust monorepo to support a public API initiative, improving extensibility for third-party integrations', 'achievement', 1),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Architected token-based authentication and middleware layer enabling secure onboarding of third-party emulation partners into the platform ecosystem', 'achievement', 2),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Built browser-side caching and dynamic chart rendering (IndexedDB) to reduce API load and improve platform dashboard responsiveness', 'achievement', 3),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Developed infrastructure automation (AWS Systems Manager, SES, Google Groups) to streamline tenant notifications, deployment workflows, and platform communications — including Go-integrated SES email templates', 'achievement', 4),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Maintained and extended the platform UI (React/Redux) including auth flows, API integrations, and role-based access', 'responsibility', 5),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Drove QA automation (Cypress) for consortium-level acceptance testing across platform releases', 'achievement', 6),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Modernized legacy services into shared platform infrastructure, reducing operational overhead and improving release velocity', 'achievement', 7),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Supported platform releases through branch management, conflict resolution, and multi-repository integration', 'responsibility', 8),
((SELECT id FROM jobs WHERE slug = 'scala-computing'), 'Evaluated and adopted AI-assisted development tools (Cursor, Claude) to accelerate platform feature delivery', 'achievement', 9);

-- Sunbird DCIM
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'sunbird-dcim'), 'Planned and executed SaaS UI migration from ExtJS/AngularJS 1.x to ES6/Angular 2.x, reducing technical debt and improving feature velocity', 'achievement', 0),
((SELECT id FROM jobs WHERE slug = 'sunbird-dcim'), 'Established company-wide UI/UX standards and component library serving as the design system for the SaaS product suite', 'achievement', 1),
((SELECT id FROM jobs WHERE slug = 'sunbird-dcim'), 'Rebuilt desktop-era DCIM interfaces as a modern SPA, improving user onboarding and reducing support burden', 'achievement', 2),
((SELECT id FROM jobs WHERE slug = 'sunbird-dcim'), 'Co-authored RESTful API contracts (YAML/OpenAPI) standardizing client-server communication patterns across the platform', 'achievement', 3),
((SELECT id FROM jobs WHERE slug = 'sunbird-dcim'), 'Hardened application security — implemented CSRF protection and session management across multi-server (RoR, Java) SaaS backend', 'achievement', 4),
((SELECT id FROM jobs WHERE slug = 'sunbird-dcim'), 'Built cross-platform mobile companion app (Cordova/AngularJS) extending SaaS platform access to field technicians', 'achievement', 5);

-- FalconStor
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'falconstor'), 'Evaluated and selected front-end stack (AngularJS SPA) for the SaaS management console, balancing team velocity with long-term maintainability', 'achievement', 0),
((SELECT id FROM jobs WHERE slug = 'falconstor'), 'Architected multi-tiered RESTful proxy API layer decoupling the SaaS UI from backend storage services', 'achievement', 1),
((SELECT id FROM jobs WHERE slug = 'falconstor'), 'Built a real-time, widget-based monitoring dashboard (Highcharts, Bootstrap) providing SaaS customers visibility into storage health and performance', 'achievement', 2),
((SELECT id FROM jobs WHERE slug = 'falconstor'), 'Designed i18n framework (JSON-based label/error-code mapping) enabling multi-region SaaS deployment', 'achievement', 3),
((SELECT id FROM jobs WHERE slug = 'falconstor'), 'Delivered mobile-responsive SaaS UI (SASS/SCSS) and hybrid mobile apps (Cordova) for iOS/Android', 'achievement', 4);

-- GalaxE.Solutions (parent + sub-engagements)
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'Screened UI/UXD candidates onshore and offshore for client engagements', 'responsibility', 0),
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'Provided optimization strategies and architectural solutions across multiple concurrent projects', 'responsibility', 1),
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'Led onshore and offshore teams to focused and successful deliveries; built lasting client relationships', 'achievement', 2),
-- Coach sub-engagement
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'Coach (April 2013 – April 2014): Managed Front-End UI/UX in jQuery for corporate website redesign; unified 10+ templates into one versatile JSP/JSTL Spring template', 'sub-engagement', 3),
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'Coach: Strategic phased implementation using SoC of DOM and Script; prototypes, Sprites, media queries, cross-browser implementations', 'sub-engagement', 4),
-- GSI Commerce sub-engagement
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'GSI Commerce (February 2011 – January 2013): Led front-end team (5 Java + 5 Web developers) for Core Services V9 — GSI Commerce''s SaaS e-commerce platform serving enterprise retailers', 'sub-engagement', 5),
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'GSI Commerce: Integrated Bazaarvoice SaaS ratings/reviews across client storefronts, email campaigns, and SEO — a turnkey multi-tenant feature rollout', 'sub-engagement', 6),
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'GSI Commerce: Built personalized product recommendation engine deployed across SaaS clients including shop.mlb.com, dickssportinggoods.com, toysrus.com', 'sub-engagement', 7),
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'GSI Commerce: Executed Q4 front-end performance sprint (minification, concatenation, CSS Sprites) improving page-load times across the SaaS client portfolio', 'sub-engagement', 8),
-- TrueAction sub-engagement
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'TrueAction (March 2010 – February 2011): Led 7-person performance team optimizing page-load times across SaaS e-commerce clients: nascar.com, petsmart.com, toysrus.com, radioshack.com', 'sub-engagement', 9),
((SELECT id FROM jobs WHERE slug = 'galaxe-solutions'), 'TrueAction: Achieved 1+ second page-load improvement for shop.nascar.com; resolved 100+ visual defects for timberland.com platform redesign', 'sub-engagement', 10);

-- Independent Contractor
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'independent-contractor'), 'Art Collection Management: Custom jQuery/PHP5/MySQL CMS with AJAX, JSON, Flash/XML, custom DHTML carousels, and image/video upload', 'achievement', 0),
((SELECT id FROM jobs WHERE slug = 'independent-contractor'), 'Talent Portfolio & Casting Sheet Management: Full front and back-end CMS with casting sheet generation, email workflow, and advanced search', 'achievement', 1),
((SELECT id FROM jobs WHERE slug = 'independent-contractor'), 'Developed multiple website initiatives for Jersey Ice Corp. with Google AdSense, Amazon aStore, and RESTful integrations', 'achievement', 2);

-- WBGO
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'wbgo'), 'Created wireframes, prototypes and demos for Web group, Strategy Group, and Board meetings', 'responsibility', 0),
((SELECT id FROM jobs WHERE slug = 'wbgo'), 'Built dynamic data-driven websites using DHTML, ASP, PHP5, SQL, XML, ActionScript, and JavaScript', 'achievement', 1),
((SELECT id FROM jobs WHERE slug = 'wbgo'), 'Provided help-desk, networking, server and computer support for ~100 employees', 'responsibility', 2);

-- Logistics.com
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'logistics-com'), 'Developed browser and platform-agnostic UI using JavaScript, DHTML, and CSS', 'achievement', 0),
((SELECT id FROM jobs WHERE slug = 'logistics-com'), 'Defined graphic design, layouts, and style guides; researched UI internationalization strategies', 'achievement', 1);

-- Openpages
INSERT OR IGNORE INTO job_details (job_id, detail_text, category, sort_order) VALUES
((SELECT id FROM jobs WHERE slug = 'openpages'), 'Managed web presence and internal tools for Openpages, an early SaaS governance/risk/compliance (GRC) platform (later acquired by IBM)', 'responsibility', 0);
