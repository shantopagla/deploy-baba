-- platform-architecture evidence
INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'platform-architecture'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'scala-computing') AND sort_order = 0),
    NULL, 0;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'platform-architecture'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'scala-computing') AND sort_order = 1),
    NULL, 1;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'platform-architecture'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'scala-computing') AND sort_order = 2),
    NULL, 2;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'platform-architecture'),
    (SELECT id FROM jobs WHERE slug = 'sunbird-dcim'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'sunbird-dcim') AND sort_order = 3),
    NULL, 3;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'platform-architecture'),
    (SELECT id FROM jobs WHERE slug = 'falconstor'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'falconstor') AND sort_order = 1),
    NULL, 4;

-- cloud-infrastructure evidence
INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'cloud-infrastructure'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'scala-computing') AND sort_order = 4),
    NULL, 0;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'cloud-infrastructure'),
    (SELECT id FROM jobs WHERE slug = 'personal-projects'),
    NULL,
    'Built zero-cost AWS deployment automation for deploy-baba: Lambda + EFS + S3 + CloudFront via Terraform',
    1;

-- frontend-engineering evidence
INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'frontend-engineering'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'scala-computing') AND sort_order = 5),
    NULL, 0;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'frontend-engineering'),
    (SELECT id FROM jobs WHERE slug = 'sunbird-dcim'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'sunbird-dcim') AND sort_order = 0),
    NULL, 1;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'frontend-engineering'),
    (SELECT id FROM jobs WHERE slug = 'sunbird-dcim'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'sunbird-dcim') AND sort_order = 1),
    NULL, 2;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'frontend-engineering'),
    (SELECT id FROM jobs WHERE slug = 'falconstor'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'falconstor') AND sort_order = 0),
    NULL, 3;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'frontend-engineering'),
    (SELECT id FROM jobs WHERE slug = 'falconstor'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'falconstor') AND sort_order = 2),
    NULL, 4;

-- ai-augmented-dev evidence
INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'ai-augmented-dev'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'scala-computing') AND sort_order = 9),
    NULL, 0;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'ai-augmented-dev'),
    (SELECT id FROM jobs WHERE slug = 'personal-projects'),
    NULL,
    'Architected AI-driven modular planning workflow for deploy-baba using Claude + plans/ system',
    1;

-- technical-leadership evidence
INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'technical-leadership'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    NULL,
    'Director, Platform Operations at Scala Computing — led platform operations strategy and team',
    0;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'technical-leadership'),
    (SELECT id FROM jobs WHERE slug = 'falconstor'),
    NULL,
    'GUI Manager at FalconStor — owned full UI lifecycle, team coordination, and delivery management',
    1;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'technical-leadership'),
    (SELECT id FROM jobs WHERE slug = 'galaxe-solutions'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'galaxe-solutions') AND sort_order = 0),
    NULL, 2;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'technical-leadership'),
    (SELECT id FROM jobs WHERE slug = 'galaxe-solutions'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'galaxe-solutions') AND sort_order = 5),
    NULL, 3;

-- saas-product evidence
INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'saas-product'),
    (SELECT id FROM jobs WHERE slug = 'scala-computing'),
    NULL,
    'Cloud-native simulation SaaS/PaaS platform — multi-tenant APIs, auth, infrastructure automation',
    0;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'saas-product'),
    (SELECT id FROM jobs WHERE slug = 'sunbird-dcim'),
    NULL,
    'DCIM SaaS platform — unified SPA, mobile companion app, OpenAPI contracts, security hardening',
    1;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'saas-product'),
    (SELECT id FROM jobs WHERE slug = 'falconstor'),
    NULL,
    'FreeStor SDS SaaS management console — real-time dashboard, i18n, mobile-responsive UI',
    2;

INSERT OR IGNORE INTO competency_evidence (competency_id, job_id, detail_id, highlight_text, sort_order)
SELECT
    (SELECT id FROM competencies WHERE slug = 'saas-product'),
    (SELECT id FROM jobs WHERE slug = 'galaxe-solutions'),
    (SELECT id FROM job_details WHERE job_id = (SELECT id FROM jobs WHERE slug = 'galaxe-solutions') AND sort_order = 5),
    NULL, 3;
