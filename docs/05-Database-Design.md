# 05 — Database Design Document

## VigilantAI: Unified Database Architecture for Real-Time Video Analytics Surveillance

**Document Classification:** Confidential — Internal Engineering  
**Version:** 1.0  
**Date:** 2026-07-21  
**Status:** Draft  
**Supersedes:** None  
**Author:** VigilantAI Architecture Team  
**Approved By:** Chief Technology Officer  
**Review Cycle:** Quarterly or upon material change to data model  

---

## Table of Contents

1. [Document Control](#1-document-control)
2. [Purpose and Scope](#2-purpose-and-scope)
3. [Definitions, Acronyms, and Abbreviations](#3-definitions-acronyms-and-abbreviations)
4. [References](#4-references)
5. [Database Design Principles](#5-database-design-principles)
6. [Database Strategy and Technology Selection](#6-database-strategy-and-technology-selection)
7. [Data Architecture Overview](#7-data-architecture-overview)
8. [Conceptual Data Model](#8-conceptual-data-model)
9. [Entity Relationship Diagrams](#9-entity-relationship-diagrams)
10. [Logical Data Model](#10-logical-data-model)
11. [Physical Data Model](#11-physical-data-model)
12. [Data Dictionary](#12-data-dictionary)
13. [Normalization and Denormalization Strategy](#13-normalization-and-denormalization-strategy)
14. [Data Lifecycle Management](#14-data-lifecycle-management)
15. [Indexing Strategy](#15-indexing-strategy)
16. [Partitioning Strategy](#16-partitioning-strategy)
17. [Replication and High Availability](#17-replication-and-high-availability)
18. [Backup and Recovery](#18-backup-and-recovery)
19. [Data Security and Encryption](#19-data-security-and-encryption)
20. [Data Validation and Integrity Constraints](#20-data-validation-and-integrity-constraints)
21. [Transaction Management and Concurrency Control](#21-transaction-management-and-concurrency-control)
22. [Query Performance and Optimization](#22-query-performance-and-optimization)
23. [Connection Pooling and Resource Management](#23-connection-pooling-and-resource-management)
24. [Data Migration and Versioning](#24-data-migration-and-versioning)
25. [Disaster Recovery and Business Continuity](#25-disaster-recovery-and-business-continuity)
26. [Monitoring and Observability](#26-monitoring-and-observability)
27. [Data Governance and Compliance](#27-data-governance-and-compliance)
28. [Scalability and Capacity Planning](#28-scalability-and-capacity-planning)
29. [Multi-Tenancy Considerations](#29-multi-tenancy-considerations)
30. [Audit Trail and Data Lineage](#30-audit-trail-and-data-lineage)
31. [Testing and Quality Assurance](#31-testing-and-quality-assurance)
32. [Risk Assessment and Mitigation](#32-risk-assessment-and-mitigation)
33. [Appendices](#33-appendices)

---

## 1. Document Control

### 1.1 Version History

| Version | Date       | Author                    | Change Summary                              |
|---------|------------|---------------------------|---------------------------------------------|
| 0.1     | 2026-06-01 | Architecture Team         | Initial draft                               |
| 0.2     | 2026-06-15 | Database Architect        | Entity model refinement                     |
| 0.3     | 2026-07-01 | Security Architect        | Encryption and compliance additions         |
| 1.0     | 2026-07-21 | Architecture Team         | Full release for review                     |

### 1.2 Distribution List

| Role                       | Name              | Access Level |
|----------------------------|-------------------|--------------|
| Chief Technology Officer   | Architecture Team | Approve      |
| Chief Information Security Officer | Security Team | Review    |
| VP of Engineering          | Engineering Team  | Review       |
| Database Architect         | Design Team       | Edit         |
| QA Lead                    | Quality Team      | Review       |
| DevOps Lead                | Operations Team   | Review       |
| Legal Counsel              | Legal Team        | Review       |

### 1.3 Review Schedule

| Review Type       | Frequency        | Responsible Party         |
|-------------------|------------------|---------------------------|
| Technical Review  | Quarterly        | Database Architect        |
| Security Review   | Semi-Annual      | CISO                      |
| Compliance Review | Annual           | Legal Counsel             |
| Architecture Review | Upon Material Change | Chief Architect    |

---

## 2. Purpose and Scope

### 2.1 Purpose

This document defines the complete database architecture and data management strategy for the VigilantAI unified real-time video analytics surveillance platform. It establishes the authoritative data model covering entity definitions, relationships, constraints, lifecycle policies, security controls, performance optimization strategies, and operational procedures required to support the platform's mission of real-time threat detection, intelligent alerting, incident management, and evidence preservation.

The database design serves as the foundational data layer that supports the software architecture defined in Document 04, implements the functional and non-functional requirements specified in Document 03, and enables the business objectives established in Document 02.

### 2.2 Scope

The scope of this document encompasses:

**In Scope:**
- Complete conceptual, logical, and physical data models for all platform domains
- Entity definitions with attributes, data types, constraints, and relationships
- Dual-database strategy covering SQLite (MVP) and PostgreSQL (Production)
- Data lifecycle management from ingestion through archival and deletion
- Indexing, partitioning, and query optimization strategies
- Data security including encryption at rest and in transit
- Backup, recovery, and disaster recovery procedures
- Data migration and schema versioning strategies
- Performance monitoring and capacity planning
- Compliance requirements for surveillance data handling
- Multi-site data isolation and geographic distribution
- Audit trail and data lineage tracking

**Out of Scope:**
- Application-layer code and API endpoint definitions (covered in Document 04)
- Infrastructure deployment and cloud configuration (covered in Document 06)
- AI/ML model storage and inference pipelines (covered in Document 04)
- Real-time data streaming architecture (covered in Document 04)
- User interface design and interaction patterns (covered in Document 07)
- Physical security of data center facilities
- Third-party database-as-a-service configurations
- ETL pipeline implementation details

### 2.3 Audience

| Audience                   | Primary Interest                                           |
|----------------------------|-----------------------------------------------------------|
| Database Architects        | Schema design, normalization, performance optimization    |
| Backend Engineers          | Entity definitions, query patterns, data access patterns  |
| Security Engineers         | Encryption, access control, audit trail design            |
| DevOps/SRE Engineers       | Backup procedures, replication, monitoring, operations    |
| QA Engineers               | Test data management, integrity validation                |
| Compliance Officers        | Data retention, access logging, regulatory adherence      |
| Product Managers           | Data capabilities, storage constraints, lifecycle rules   |
| Legal Counsel              | Evidence handling, chain of custody, data sovereignty     |

---

## 3. Definitions, Acronyms, and Abbreviations

| Term                   | Definition                                                       |
|------------------------|------------------------------------------------------------------|
| ACID                   | Atomicity, Consistency, Isolation, Durability — database transaction properties |
| ACL                    | Access Control List                                              |
| API                    | Application Programming Interface                                |
| ARN                    | Amazon Resource Name                                             |
| BLOB                   | Binary Large Object                                              |
| B-Tree                 | Balanced Tree — standard database index structure                |
| C4 Model               | Context, Container, Component, Code — architecture model levels |
| CDC                    | Change Data Capture                                              |
| CI/CD                  | Continuous Integration / Continuous Deployment                   |
| DDL                    | Data Definition Language                                         |
| DML                    | Data Manipulation Language                                       |
| DNS                    | Domain Name System                                               |
| EAV                    | Entity-Attribute-Value                                           |
| ER Diagram             | Entity Relationship Diagram                                     |
| GDPR                   | General Data Protection Regulation (EU)                          |
| GIN                    | Generalized Inverted Index (PostgreSQL)                          |
| GTIN                   | Global Transaction Identifier                                    |
| HIPAA                  | Health Insurance Portability and Accountability Act              |
| HMAC                   | Hash-based Message Authentication Code                           |
| HSM                    | Hardware Security Module                                         |
| IOPS                   | Input/Output Operations Per Second                               |
| JSONB                  | Binary JSON (PostgreSQL)                                         |
| JWT                    | JSON Web Token                                                   |
| MFA                    | Multi-Factor Authentication                                      |
| MVP                    | Minimum Viable Product                                           |
| NVMe                   | Non-Volatile Memory Express                                      |
| OLTP                   | Online Transaction Processing                                    |
| ORM                    | Object-Relational Mapping                                        |
| PII                    | Personally Identifiable Information                               |
| RBAC                   | Role-Based Access Control                                        |
| RDS                    | Amazon Relational Database Service                               |
| RPO                    | Recovery Point Objective                                         |
| RTO                    | Recovery Time Objective                                          |
| S3                     | Amazon Simple Storage Service                                    |
| SHA-256                | Secure Hash Algorithm (256-bit)                                  |
| SQLx                   | Rust SQL toolkit with compile-time query checking                |
| SSL/TLS                | Secure Sockets Layer / Transport Layer Security                  |
| WAL                    | Write-Ahead Logging                                              |
| X.509                  | Public key certificate standard                                  |

---

## 4. References

### 4.1 VigilantAI Project Documents

| ID   | Document Title                        | Version | Status |
|------|---------------------------------------|---------|--------|
| 01   | Executive Summary                     | 1.0     | Draft  |
| 02   | Business Requirements                 | 1.0     | Draft  |
| 03   | System Requirements Specification     | 1.0     | Draft  |
| 04   | Software Architecture                 | 1.0     | Draft  |

### 4.2 External Standards and References

| Standard/Reference                                      | Relevance                                              |
|---------------------------------------------------------|--------------------------------------------------------|
| NIST SP 800-53 (Rev. 5)                                 | Security and privacy controls for information systems  |
| ISO/IEC 27001:2022                                       | Information security management systems                |
| ISO/IEC 27002:2022                                       | Information security controls                          |
| GDPR (Regulation 2016/679)                               | EU data protection and privacy                         |
| CCPA (California Consumer Privacy Act)                   | US consumer privacy rights                             |
| BIP-0035 (Bitcoin Transaction Malleability)              | Transaction integrity references                      |
| NIST FIPS 140-3                                          | Cryptographic module validation                        |
| PCI DSS v4.0                                              | Payment card data security (if applicable)             |
| OWASP ASVS v4.0                                          | Application security verification                      |
| PostgreSQL Documentation (v16)                            | PostgreSQL feature reference                           |
| SQLite Documentation                                     | SQLite feature reference                               |
| SQLx Documentation (Rust)                                | Compile-time checked SQL queries                       |
| RFC 7231 (HTTP/1.1)                                      | HTTP semantics for API layer                            |
| RFC 7519 (JWT)                                           | JSON Web Token specification                           |
| RFC 4180 (CSV)                                           | Comma-separated values format                          |

---

## 5. Database Design Principles

### 5.1 Core Design Principles

The VigilantAI database architecture is governed by twelve foundational principles that ensure data integrity, operational reliability, security, and long-term maintainability.

#### Principle 1: Data Integrity is Non-Negotiable

Every database design decision must preserve the accuracy, consistency, and reliability of data. The system shall enforce referential integrity, domain constraints, and business rules at the database level — never relying solely on application-layer validation. Data integrity is the foundation upon which all other properties depend. No performance optimization, convenience feature, or architectural shortcut shall compromise data integrity.

#### Principle 2: Defense in Depth for Data Protection

Data protection must be implemented in multiple layers: encryption at rest, encryption in transit, access control at the database level, application-level authorization, audit logging of all data access, and physical security of storage media. No single point of failure in the security architecture shall expose sensitive data.

#### Principle 3: Schema Design Reflects Business Reality

The data model must accurately represent the business domain, its entities, relationships, and rules. Schema design shall not be distorted by short-term technical constraints or ORM limitations. The database schema is the authoritative representation of the business data model.

#### Principle 4: Progressive Complexity

The database architecture shall support progressive complexity: starting with SQLite for development and MVP deployment, scaling to PostgreSQL for production, with transparent migration paths. Application code shall remain database-agnostic through the Repository Pattern, enabling seamless transition between database engines without modifying business logic.

#### Principle 5: Temporal Awareness

All data entities shall support temporal tracking: creation timestamps, modification timestamps, soft-deletion timestamps, and version history where required. The system must be able to reconstruct the state of any entity at any point in time for forensic and compliance purposes.

#### Principle 6: Data Lifecycle Ownership

Every data entity shall have a clearly defined lifecycle: creation, active use, archival, and deletion. Data retention policies shall be enforceable at the database level through automated procedures. No data shall persist beyond its authorized retention period.

#### Principle 7: Performance Through Intelligent Design

Performance shall be achieved through proper schema design, strategic indexing, appropriate normalization/denormalization balance, and efficient query patterns — not through premature optimization or speculative denormalization. Every index must have a justified use case backed by query analysis.

#### Principle 8: Observability by Default

All database operations shall generate sufficient telemetry to enable monitoring, performance analysis, security auditing, and troubleshooting. Database observability is not optional — it is a core operational requirement.

#### Principle 9: Resilience and Recoverability

The database architecture shall ensure that data survives hardware failures, software bugs, operator errors, and disaster scenarios. Backup, recovery, and replication strategies shall be tested regularly and validated through documented procedures.

#### Principle 10: Compliance by Design

Regulatory compliance requirements (GDPR, CCPA, HIPAA, SOX) shall be embedded in the database design through data classification, access controls, retention policies, and audit capabilities — not bolted on after deployment.

#### Principle 11: Zero Trust Data Access

Every data access request shall be authenticated, authorized, and logged. The database shall not trust any client, connection, or application implicitly. Least-privilege principles shall govern all database accounts and connection permissions.

#### Principle 12: Documentation as Code

The data model, schema definitions, migration scripts, and operational procedures shall be maintained in version control, peer-reviewed, and treated as first-class engineering artifacts. Database changes follow the same quality gates as application code.

---

### 5.2 Design Decision Framework

Every database design decision shall be evaluated against the following decision framework:

| Criterion              | Weight | Evaluation Questions                                          |
|------------------------|--------|---------------------------------------------------------------|
| Data Integrity         | 25%    | Does this preserve accuracy and consistency?                  |
| Security               | 20%    | Does this protect data from unauthorized access?              |
| Performance            | 15%    | Does this meet latency and throughput requirements?           |
| Scalability            | 15%    | Does this support growth from 50 to 10,000+ cameras?         |
| Maintainability        | 10%    | Can this be understood, modified, and operated by the team?   |
| Cost                   | 10%    | Does this balance capability with resource efficiency?        |
| Compliance             | 5%     | Does this satisfy regulatory requirements?                    |

---

## 6. Database Strategy and Technology Selection

### 6.1 Dual-Database Architecture Rationale

VigilantAI employs a dual-database strategy to balance development velocity with production-grade reliability. This approach recognizes that the database requirements for a proof-of-concept MVP with 50–200 cameras differ materially from those of a production deployment serving 500–10,000+ cameras across multiple geographic regions.

The Repository Pattern (defined in Document 04, Section 7.6) provides the abstraction layer that enables transparent migration between database engines. Application code interacts exclusively with repository interfaces; the underlying database implementation is a deployment-time decision that does not propagate into business logic.

### 6.2 SQLite — Minimum Viable Product

| Attribute                  | Detail                                                        |
|----------------------------|---------------------------------------------------------------|
| Version                    | SQLite 3.45+                                                  |
| Use Case                   | MVP, development, testing, single-site deployments            |
| Target Scale               | 50–200 cameras, single-server deployment                      |
| Concurrency Model          | Single-writer, multiple-reader                                |
| Storage Engine             | B-Tree (WAL mode)                                             |
| Max Database Size          | 281 TB (theoretical), practical limit ~1 TB                   |
| Connection Model           | In-process, zero-copy                                         |
| Deployment Complexity      | Minimal — single file, no server process                      |
| Backup Strategy            | File-level copy + WAL checkpoint                              |
| Replication                | Not built-in; application-level synchronization              |
| Full-Text Search           | FTS5 extension                                                |
| JSON Support               | JSON1 extension                                               |
| Encryption                 | SQLCipher extension (AES-256)                                 |

**SQLite Selection Criteria:**
- Zero operational overhead for MVP and development environments
- Single-file deployment simplifies distribution and backup
- WAL mode provides adequate concurrency for single-server deployments
- FTS5 extension supports evidence content search
- SQLCipher provides transparent encryption at rest
- Proven reliability in embedded and edge computing scenarios

**SQLite Limitations Acknowledged:**
- Single-writer concurrency limits throughput under heavy concurrent writes
- No native replication or high-availability mechanisms
- Limited role-based access control compared to PostgreSQL
- No stored procedures or advanced query optimization features
- File-level locking may cause contention under extreme concurrent load

### 6.3 PostgreSQL — Production Deployment

| Attribute                  | Detail                                                        |
|----------------------------|---------------------------------------------------------------|
| Version                    | PostgreSQL 16+                                                |
| Use Case                   | Production, multi-site, enterprise deployments                |
| Target Scale               | 500–10,000+ cameras, distributed deployment                   |
| Concurrency Model          | MVCC (Multi-Version Concurrency Control)                      |
| Storage Engine             | Heap-organized tables with TOAST for large values             |
| Max Database Size          | Unlimited (practical limit determined by hardware)            |
| Connection Model           | Client-server with connection pooling (PgBouncer)              |
| Deployment Complexity      | Moderate — requires server process, configuration, monitoring |
| Backup Strategy            | pg_dump, pg_basebackup, WAL archiving, Point-in-Time Recovery |
| Replication                | Streaming replication, logical replication                    |
| Full-Text Search           | Built-in tsvector/tsquery                                     |
| JSON Support               | JSONB with indexing                                            |
| Encryption                 | pgcrypto, TLS, Transparent Data Encryption (TDE)              |

**PostgreSQL Selection Criteria:**
- Mature MVCC implementation supports high-concurrency multi-camera deployments
- Native streaming replication enables high availability and read scaling
- Row-level security policies enforce data isolation without application code
- JSONB columns support semi-structured data without schema rigidity
- Partitioning enables time-based data management for detection events and audit logs
- Advanced indexing (GIN, GiST, BRIN) supports complex query patterns
- Stored procedures in PL/pgSQL enable complex business rule enforcement
- Extensive ecosystem of monitoring, backup, and operational tools

### 6.4 Database Engine Feature Comparison

| Feature                          | SQLite           | PostgreSQL         |
|----------------------------------|------------------|--------------------|
| ACID Transactions                | Yes (WAL)        | Yes (MVCC)         |
| Referential Integrity            | Yes              | Yes                |
| Role-Based Access Control        | Limited          | Full (RLS)         |
| Row-Level Security               | No               | Yes                |
| Streaming Replication            | No               | Yes                |
| Logical Replication              | No               | Yes                |
| Partitioning                     | No               | Yes (Declarative)  |
| Parallel Query Execution         | No               | Yes                |
| Connection Pooling               | N/A (in-process) | Yes (PgBouncer)    |
| Stored Procedures                | No               | Yes (PL/pgSQL)     |
| Advanced Indexing (GIN/GiST)     | Limited          | Full               |
| JSON/JSONB Support               | JSON1 extension  | Native JSONB       |
| Full-Text Search                 | FTS5             | Built-in           |
| Transparent Data Encryption      | SQLCipher         | TDE (via extension)|
| Point-in-Time Recovery           | No               | Yes                |
| Table Partitioning               | No               | Yes (Declarative)  |
| Materialized Views               | No               | Yes                |
| Foreign Data Wrappers            | No               | Yes                |
| Event Triggers                   | No               | Yes                |
| Custom Types/Enums               | Limited          | Full               |

### 6.5 Repository Pattern — Database Abstraction

The Repository Pattern (Document 04, Section 7.6) ensures that application code remains database-agnostic. The abstraction operates at the following levels:

**Interface Layer:** Application code depends only on repository trait definitions (Rust traits or Python abstract base classes). These interfaces define data access operations without database-specific implementations.

**Implementation Layer:** Concrete repository implementations exist for each database engine. The SQLite implementation uses rusqlite or SQLx with SQLite driver; the PostgreSQL implementation uses SQLx with PostgreSQL driver. Both implement identical repository interfaces.

**Query Layer:** SQLx provides compile-time checked queries for both database engines. Query syntax differences are encapsulated within repository implementations. Parameter binding, result mapping, and error handling follow database-agnostic patterns.

**Migration Layer:** Database schema migrations are versioned and engine-specific. Migration scripts exist for both SQLite and PostgreSQL, ensuring schema parity across environments. The migration framework tracks applied migrations and supports rollback.

**Connection Layer:** Connection management is abstracted to support in-process connections (SQLite) and client-server connections (PostgreSQL) uniformly. Connection pooling is managed at the implementation layer.

### 6.6 Technology Justification Summary

| Decision                        | Rationale                                                              |
|---------------------------------|------------------------------------------------------------------------|
| Relational Database (not NoSQL) | Structured data with complex relationships; ACID requirements; SQL ecosystem maturity |
| SQLx (not Diesel/SeaORM)       | Compile-time query verification; minimal runtime overhead; full SQL control |
| Repository Pattern (not Direct) | Database-agnostic application code; transparent engine migration       |
| SQLite (not embedded key-value)| Full relational features; ACID compliance; familiar SQL semantics      |
| PostgreSQL (not MySQL)         | Superior JSONB support; row-level security; partitioning; MVCC maturity |
| pgBouncer (not built-in pooling)| Battle-tested connection pooling; separate lifecycle from DB server   |
| SQLCipher (not app-level encrypt)| Transparent encryption; minimal performance overhead; proven implementation |

---

## 7. Data Architecture Overview

### 7.1 Data Domain Decomposition

The VigilantAI data model is organized into seven distinct data domains, each representing a cohesive business capability with clear ownership, boundaries, and access patterns.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          VigilantAI Data Domains                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │   Identity   │  │   Physical   │  │   Event      │  │   Response   │   │
│  │   & Access   │  │   Assets     │  │   Processing │  │   Management │   │
│  │              │  │              │  │              │  │              │   │
│  │ • Users      │  │ • Sites      │  │ • Detections │  │ • Incidents  │   │
│  │ • Roles      │  │ • Cameras    │  │ • Rules      │  │ • Evidence   │   │
│  │ • Permissions│  │ • Camera Grps│  │ • Events     │  │ • Notes      │   │
│  │ • Sessions   │  │ • Health     │  │ • Alerts     │  │ • Actions    │   │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘   │
│                                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────────────┐  │
│  │  Analytics   │  │ Communication│  │         System & Operations      │  │
│  │  & Reporting │  │ & Notification│ │                                  │  │
│  │              │  │              │  │ • System Configuration           │  │
│  │ • Reports    │  │ • Notif Rules│  │ • Audit Logs                     │  │
│  │ • Dashboards │  │ • Notif Hist │  │ • Health Metrics                 │  │
│  │ • KPIs       │  │ • Channels   │  │ • API Keys                       │  │
│  └──────────────┘  └──────────────┘  └──────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 7.2 Domain Ownership and Boundaries

| Domain                     | Owner                    | Core Entities                                    | Data Sensitivity |
|----------------------------|--------------------------|--------------------------------------------------|------------------|
| Identity & Access          | Security Team            | Users, Roles, Permissions, Sessions, API Keys    | HIGH             |
| Physical Assets            | Infrastructure Team      | Sites, Camera Groups, Cameras, Camera Health     | MEDIUM           |
| Event Processing           | AI/ML Team               | Detection Events, Rules, Event Metadata           | MEDIUM           |
| Response Management        | Operations Team          | Incidents, Evidence, Evidence Hash, Notes         | HIGH             |
| Analytics & Reporting      | Product Team             | Reports, Dashboard Preferences, KPIs             | LOW              |
| Communication              | Operations Team          | Notification Rules, Notification History, Channels| MEDIUM           |
| System & Operations        | Platform Team            | System Config, Audit Logs, Health Metrics        | HIGH             |

### 7.3 Cross-Domain Relationships

The seven data domains are interconnected through well-defined foreign key relationships and event-driven data flows. Understanding these cross-domain relationships is essential for maintaining referential integrity and designing efficient queries.

**Identity & Access → Physical Assets:** Users are assigned to Sites through Site Permissions; Camera Group access is governed by Role-based permissions.

**Physical Assets → Event Processing:** Cameras generate Detection Events; Camera Groups define monitoring scopes for Rules.

**Event Processing → Response Management:** Detection Events trigger Incidents; Rules determine which events escalate to Incidents.

**Response Management → Identity & Access:** Incidents are assigned to Users; Evidence access is controlled by User Permissions.

**Communication → Identity & Access:** Notifications are delivered to Users based on Role assignments.

**System & Operations → All Domains:** Audit Logs capture all data modifications across all domains; System Configuration governs global platform behavior.

### 7.4 Data Flow Architecture

```
┌────────────────────────────────────────────────────────────────────────────┐
│                          Data Flow Architecture                             │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Camera Feed → AI Pipeline → Detection Events → Rule Engine → Incidents   │
│       │              │              │                │             │       │
│       ▼              ▼              ▼                ▼             ▼       │
│  Camera Health   Event Metadata  Event Storage   Match Results  Evidence  │
│       │              │              │                │             │       │
│       ▼              ▼              ▼                ▼             ▼       │
│  Health Metrics  Audit Logs   Analytics DB    Notifications   Chain of    │
│                                          │                   Custody      │
│                                          ▼                              │
│                                    Dashboard / Reports                   │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 7.5 Data Classification Framework

All data entities are classified according to sensitivity and regulatory requirements:

| Classification   | Description                                               | Examples                          | Controls Required                    |
|------------------|-----------------------------------------------------------|-----------------------------------|--------------------------------------|
| RESTRICTED       | Highly sensitive data subject to strict regulatory controls| PII, authentication credentials, API keys | Encryption (AES-256), access logging, MFA, short retention |
| CONFIDENTIAL     | Business-sensitive data with limited distribution         | Evidence clips, incident details, audit logs | Encryption, RBAC, audit logging, medium retention |
| INTERNAL         | Data for internal operational use                         | Camera configurations, health metrics, system config | RBAC, standard access controls |
| PUBLIC           | Data that may be shared externally                        | Published reports, public dashboards | Integrity controls, versioning |

---

## 8. Conceptual Data Model

### 8.1 Entity Identification

The conceptual data model identifies twenty-one core entities across the seven data domains. Each entity represents a distinct business object with unique identity, attributes, and lifecycle.

| Entity                    | Domain                 | Description                                                        | Lifecycle        |
|---------------------------|------------------------|--------------------------------------------------------------------|------------------|
| User                      | Identity & Access      | Individual with platform access credentials                        | Active → Inactive → Deleted |
| Role                      | Identity & Access      | Named collection of permissions                                    | Active → Deprecated |
| Permission                | Identity & Access      | Granular access right for specific resources and actions           | Active → Deprecated |
| User Session              | Identity & Access      | Active authentication session for a user                           | Active → Expired |
| API Key                   | Identity & Access      | Programmatic access credentials for integrations                   | Active → Revoked |
| Site                      | Physical Assets        | Physical location monitored by the platform                       | Active → Inactive |
| Camera Group              | Physical Assets        | Logical grouping of cameras for organizational purposes            | Active → Archived |
| Camera                    | Physical Assets        | Video capture device managed by the platform                      | Active → Offline → Decommissioned |
| Camera Health             | Physical Assets        | Time-series health and connectivity metrics for cameras            | Current → Historical |
| Detection Event           | Event Processing       | AI-generated detection of objects, persons, or behaviors           | New → Processed → Archived |
| Rule                      | Event Processing       | Conditional logic that triggers alerts based on detection criteria | Active → Disabled |
| Alert                     | Event Processing       | Notification triggered by a rule match                             | New → Acknowledged → Resolved |
| Incident                  | Response Management    | Operational response to a significant security event               | Open → Investigating → Closed |
| Evidence                  | Response Management    | Video clip, image, or document supporting an incident              | Captured → Verified → Archived |
| Evidence Hash             | Response Management    | Cryptographic hash ensuring evidence integrity                     | Created → Verified |
| Report                    | Analytics & Reporting  | Generated analysis output for specified time periods               | Draft → Published |
| Dashboard Preference      | Analytics & Reporting  | User-specific dashboard configuration and layout                   | Active → Updated |
| Notification Rule         | Communication          | Configuration defining when and how notifications are sent         | Active → Disabled |
| Notification History      | Communication          | Log of all notifications sent through the platform                | Sent → Delivered → Failed |
| System Configuration      | System & Operations    | Platform-wide settings and feature flags                           | Active → Updated |
| Audit Log                 | System & Operations    | Immutable record of all significant system actions                 | Created → Archived |

### 8.2 Conceptual Relationship Summary

| Relationship                              | Cardinality  | Description                                           |
|-------------------------------------------|--------------|-------------------------------------------------------|
| User → Role                               | M:N          | Users are assigned multiple Roles                     |
| Role → Permission                         | M:N          | Roles contain multiple Permissions                    |
| User → User Session                       | 1:N          | Users have multiple concurrent Sessions               |
| User → Site                               | M:N          | Users are granted access to specific Sites            |
| Site → Camera Group                       | 1:N          | Sites contain multiple Camera Groups                  |
| Camera Group → Camera                     | 1:N          | Groups contain multiple Cameras                       |
| Camera → Camera Health                    | 1:N          | Cameras have multiple Health Records                  |
| Camera → Detection Event                  | 1:N          | Cameras generate multiple Detection Events            |
| Rule → Detection Event                    | 1:N          | Rules evaluate multiple Detection Events              |
| Detection Event → Alert                   | 1:N          | Events may trigger multiple Alerts                    |
| Detection Event → Incident                | 1:N          | Events may create multiple Incidents                  |
| Incident → Evidence                       | 1:N          | Incidents contain multiple Evidence items             |
| Evidence → Evidence Hash                  | 1:1          | Each Evidence has one integrity Hash                  |
| Incident → User                           | N:1          | Incidents are assigned to responsible Users           |
| User → Report                             | 1:N          | Users generate multiple Reports                       |
| User → Notification History               | 1:N          | Users receive multiple Notifications                  |
| Notification Rule → Notification History  | 1:N          | Rules generate multiple Notification records          |
| System Configuration → All Entities       | 1:1 (global) | Global configuration applies platform-wide            |
| All Entities → Audit Log                  | 1:N          | All modifications generate Audit Log entries          |

---

## 9. Entity Relationship Diagrams

### 9.1 Identity & Access Domain ER Diagram

```mermaid
erDiagram
    USERS {
        uuid id PK
        varchar username UK
        varchar email UK
        varchar password_hash
        varchar display_name
        varchar phone
        varchar mfa_secret
        boolean mfa_enabled
        varchar status
        timestamp last_login_at
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    ROLES {
        uuid id PK
        varchar name UK
        varchar description
        boolean is_system_role
        varchar status
        timestamp created_at
        timestamp updated_at
    }

    PERMISSIONS {
        uuid id PK
        varchar resource_type
        varchar action
        varchar description
        timestamp created_at
    }

    USER_SESSIONS {
        uuid id PK
        uuid user_id FK
        varchar session_token
        varchar refresh_token
        inet ip_address
        varchar user_agent
        timestamp expires_at
        timestamp created_at
        timestamp last_activity_at
    }

    API_KEYS {
        uuid id PK
        uuid user_id FK
        varchar name
        varchar key_hash
        varchar key_prefix
        varchar scopes
        timestamp expires_at
        timestamp last_used_at
        timestamp created_at
        timestamp revoked_at
    }

    USER_ROLES {
        uuid user_id FK
        uuid role_id FK
        timestamp assigned_at
        uuid assigned_by FK
    }

    ROLE_PERMISSIONS {
        uuid role_id FK
        uuid permission_id FK
        timestamp granted_at
    }

    USER_SITE_ACCESS {
        uuid user_id FK
        uuid site_id FK
        varchar access_level
        timestamp granted_at
        uuid granted_by FK
    }

    USERS ||--o{ USER_SESSIONS : "has sessions"
    USERS ||--o{ API_KEYS : "owns api keys"
    USERS ||--o{ USER_ROLES : "assigned roles"
    ROLES ||--o{ USER_ROLES : "granted to users"
    ROLES ||--o{ ROLE_PERMISSIONS : "contains permissions"
    PERMISSIONS ||--o{ ROLE_PERMISSIONS : "granted via roles"
    USERS ||--o{ USER_SITE_ACCESS : "site access"
```

### 9.2 Physical Assets Domain ER Diagram

```mermaid
erDiagram
    SITES {
        uuid id PK
        varchar name
        varchar address
        varchar city
        varchar state
        varchar country
        varchar timezone
        decimal latitude
        decimal longitude
        varchar status
        jsonb metadata
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    CAMERA_GROUPS {
        uuid id PK
        uuid site_id FK
        varchar name
        varchar description
        varchar status
        timestamp created_at
        timestamp updated_at
    }

    CAMERAS {
        uuid id PK
        uuid site_id FK
        uuid camera_group_id FK
        varchar name
        varchar stream_url
        varchar rtsp_url
        varchar status
        integer fps
        integer resolution_width
        integer resolution_height
        boolean night_vision_enabled
        boolean motion_detection_enabled
        varchar storage_mode
        jsonb metadata
        timestamp last_health_check
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    CAMERA_HEALTH {
        uuid id PK
        uuid camera_id FK
        varchar status
        integer fps_actual
        integer bitrate_kbps
        integer latency_ms
        integer packet_loss_percent
        integer cpu_usage_percent
        integer memory_usage_mb
        integer storage_used_gb
        jsonb diagnostics
        timestamp recorded_at
    }

    SITES ||--o{ CAMERA_GROUPS : "contains groups"
    SITES ||--o{ CAMERAS : "hosts cameras"
    CAMERA_GROUPS ||--o{ CAMERAS : "organizes cameras"
    CAMERAS ||--o{ CAMERA_HEALTH : "health metrics"
```

### 9.3 Event Processing Domain ER Diagram

```mermaid
erDiagram
    DETECTION_EVENTS {
        uuid id PK
        uuid camera_id FK
        uuid rule_id FK
        varchar event_type
        varchar severity
        float confidence_score
        jsonb detected_objects
        jsonb bounding_boxes
        varchar zone_id
        jsonb metadata
        varchar processing_status
        timestamp detected_at
        timestamp processed_at
        timestamp created_at
    }

    RULES {
        uuid id PK
        uuid site_id FK
        varchar name
        varchar description
        varchar rule_type
        jsonb conditions
        jsonb actions
        varchar severity
        boolean is_enabled
        integer priority
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    ALERTS {
        uuid id PK
        uuid detection_event_id FK
        uuid rule_id FK
        varchar severity
        varchar status
        varchar message
        jsonb context
        timestamp triggered_at
        timestamp acknowledged_at
        timestamp resolved_at
        timestamp created_at
    }

    DETECTION_EVENTS ||--o{ ALERTS : "triggers alerts"
    RULES ||--o{ DETECTION_EVENTS : "evaluated against"
    RULES ||--o{ ALERTS : "generates alerts"
```

### 9.4 Response Management Domain ER Diagram

```mermaid
erDiagram
    INCIDENTS {
        uuid id PK
        uuid detection_event_id FK
        uuid assigned_user_id FK
        uuid site_id FK
        varchar title
        varchar description
        varchar severity
        varchar status
        varchar priority
        jsonb metadata
        timestamp acknowledged_at
        timestamp resolved_at
        timestamp created_at
        timestamp updated_at
    }

    EVIDENCE {
        uuid id PK
        uuid incident_id FK
        uuid camera_id FK
        varchar evidence_type
        varchar file_path
        varchar file_name
        varchar mime_type
        bigint file_size_bytes
        varchar storage_bucket
        varchar storage_key
        jsonb metadata
        timestamp captured_at
        timestamp created_at
        timestamp deleted_at
    }

    EVIDENCE_HASH {
        uuid id PK
        uuid evidence_id FK
        varchar hash_algorithm
        varchar hash_value
        varchar signature
        timestamp computed_at
        timestamp verified_at
    }

    INCIDENT_NOTES {
        uuid id PK
        uuid incident_id FK
        uuid author_user_id FK
        text content
        varchar note_type
        timestamp created_at
        timestamp updated_at
    }

    INCIDENTS ||--o{ EVIDENCE : "contains evidence"
    INCIDENTS ||--o{ INCIDENT_NOTES : "has notes"
    EVIDENCE ||--o| EVIDENCE_HASH : "integrity hash"
    EVIDENCE }o--|| CAMERAS : "captured by"
```

### 9.5 System & Operations Domain ER Diagram

```mermaid
erDiagram
    AUDIT_LOGS {
        uuid id PK
        uuid user_id FK
        varchar entity_type
        uuid entity_id
        varchar action
        jsonb old_values
        jsonb new_values
        inet ip_address
        varchar user_agent
        timestamp created_at
    }

    SYSTEM_CONFIGURATION {
        uuid id PK
        varchar config_key UK
        text config_value
        varchar config_type
        varchar description
        varchar updated_by
        timestamp updated_at
    }

    HEALTH_METRICS {
        uuid id PK
        varchar service_name
        varchar metric_name
        float metric_value
        varchar unit
        jsonb tags
        timestamp recorded_at
    }

    NOTIFICATION_RULES {
        uuid id PK
        varchar name
        varchar event_type
        varchar severity
        jsonb conditions
        jsonb channels
        jsonb recipients
        boolean is_enabled
        timestamp created_at
        timestamp updated_at
    }

    NOTIFICATION_HISTORY {
        uuid id PK
        uuid notification_rule_id FK
        uuid user_id FK
        varchar channel
        varchar status
        text subject
        text body
        jsonb metadata
        timestamp sent_at
        timestamp delivered_at
        timestamp failed_at
    }

    NOTIFICATION_RULES ||--o{ NOTIFICATION_HISTORY : "generates"
```

---

## 10. Logical Data Model

### 10.1 User Entity

The User entity represents an individual with platform access. It supports multi-factor authentication, soft deletion, and comprehensive lifecycle tracking.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| username           | VARCHAR(64)   | No       | —            | Unique login username                          |
| email              | VARCHAR(255)  | No       | —            | Unique email address                           |
| password_hash      | VARCHAR(255)  | No       | —            | bcrypt hash of password                        |
| display_name       | VARCHAR(128)  | Yes      | NULL         | Human-readable display name                    |
| phone              | VARCHAR(32)   | Yes      | NULL         | Contact phone number                           |
| mfa_secret         | VARCHAR(255)  | Yes      | NULL         | TOTP secret key for MFA                        |
| mfa_enabled        | BOOLEAN       | No       | false        | Whether MFA is enabled                         |
| status             | ENUM          | No       | 'active'     | Account status: active, inactive, locked       |
| last_login_at      | TIMESTAMP     | Yes      | NULL         | Timestamp of last successful login             |
| created_at         | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |
| updated_at         | TIMESTAMP     | No       | NOW()        | Last modification timestamp                    |
| deleted_at         | TIMESTAMP     | Yes      | NULL         | Soft deletion timestamp                        |

**Constraints:**
- UNIQUE(username)
- UNIQUE(email)
- CHECK(length(username) >= 3 AND length(username) <= 64)
- CHECK(email LIKE '%@%.%')
- CHECK(status IN ('active', 'inactive', 'locked'))

### 10.2 Role Entity

The Role entity represents a named collection of permissions that can be assigned to users. System roles are reserved and cannot be modified.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| name               | VARCHAR(64)   | No       | —            | Unique role name                               |
| description        | TEXT          | Yes      | NULL         | Human-readable role description                |
| is_system_role     | BOOLEAN       | No       | false        | Reserved system role flag                      |
| status             | ENUM          | No       | 'active'     | Role status: active, deprecated                |
| created_at         | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |
| updated_at         | TIMESTAMP     | No       | NOW()        | Last modification timestamp                    |

**Constraints:**
- UNIQUE(name)
- CHECK(status IN ('active', 'deprecated'))

**Predefined System Roles:**
| Role Name       | Description                                           |
|-----------------|-------------------------------------------------------|
| Super Admin     | Full system access including configuration            |
| Security Admin  | Security operations and incident management           |
| Security Analyst| Read-only access to events, incidents, and evidence   |
| Operator        | Operational access to cameras and live monitoring     |
| Viewer          | Read-only dashboard access                            |
| API Integration | Programmatic access for external systems              |

### 10.3 Permission Entity

The Permission entity represents a granular access right for a specific resource and action combination.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| resource_type      | VARCHAR(64)   | No       | —            | Target resource: camera, incident, evidence    |
| action             | VARCHAR(32)   | No       | —            | Action: create, read, update, delete, export   |
| description        | TEXT          | Yes      | NULL         | Human-readable permission description          |
| created_at         | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |

**Constraints:**
- UNIQUE(resource_type, action)

**Permission Matrix:**
| Resource        | Create | Read | Update | Delete | Export |
|-----------------|--------|------|--------|--------|--------|
| camera          | Yes    | Yes  | Yes    | Yes    | No     |
| incident        | Yes    | Yes  | Yes    | No     | Yes    |
| evidence        | Yes    | Yes  | No     | No     | Yes    |
| user            | Yes    | Yes  | Yes    | Yes    | No     |
| role            | Yes    | Yes  | Yes    | No     | No     |
| rule            | Yes    | Yes  | Yes    | Yes    | No     |
| report          | Yes    | Yes  | Yes    | Yes    | Yes    |
| site            | Yes    | Yes  | Yes    | No     | No     |
| configuration   | No     | Yes  | Yes    | No     | No     |
| audit_log       | No     | Yes  | No     | No     | Yes    |

### 10.4 User Session Entity

The User Session entity tracks active authentication sessions for users.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique session identifier              |
| user_id            | UUID          | No       | —            | FK to Users                                    |
| session_token      | VARCHAR(512)  | No       | —            | Opaque session token                           |
| refresh_token      | VARCHAR(512)  | No       | —            | Token for session renewal                      |
| ip_address         | INET          | Yes      | NULL         | Client IP address                              |
| user_agent         | TEXT          | Yes      | NULL         | Client user agent string                       |
| expires_at         | TIMESTAMP     | No       | —            | Session expiration timestamp                   |
| created_at         | TIMESTAMP     | No       | NOW()        | Session creation timestamp                     |
| last_activity_at   | TIMESTAMP     | No       | NOW()        | Last recorded activity                         |

**Constraints:**
- UNIQUE(session_token)
- UNIQUE(refresh_token)
- FK(user_id) REFERENCES Users(id) ON DELETE CASCADE
- CHECK(expires_at > created_at)

### 10.5 Site Entity

The Site entity represents a physical location monitored by the VigilantAI platform.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| name               | VARCHAR(128)  | No       | —            | Site name                                      |
| address            | TEXT          | Yes      | NULL         | Physical street address                        |
| city               | VARCHAR(128)  | Yes      | NULL         | City name                                      |
| state              | VARCHAR(128)  | Yes      | NULL         | State or province                              |
| country            | VARCHAR(64)   | Yes      | NULL         | Country name                                   |
| timezone           | VARCHAR(64)   | No       | 'UTC'        | IANA timezone identifier                       |
| latitude           | DECIMAL(10,8) | Yes      | NULL         | GPS latitude                                   |
| longitude          | DECIMAL(11,8) | Yes      | NULL         | GPS longitude                                  |
| status             | ENUM          | No       | 'active'     | Site status: active, inactive                  |
| metadata           | JSONB         | Yes      | '{}'         | Additional site metadata                       |
| created_at         | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |
| updated_at         | TIMESTAMP     | No       | NOW()        | Last modification timestamp                    |
| deleted_at         | TIMESTAMP     | Yes      | NULL         | Soft deletion timestamp                        |

**Constraints:**
- CHECK(status IN ('active', 'inactive'))
- CHECK(latitude >= -90 AND latitude <= 90)
- CHECK(longitude >= -180 AND longitude <= 180)
- CHECK(timezone IN ('UTC', 'US/Eastern', 'US/Central', 'US/Mountain', 'US/Pacific', 'Europe/London', 'Europe/Berlin', 'Asia/Tokyo', 'Asia/Singapore', 'Australia/Sydney'))

### 10.6 Camera Entity

The Camera entity represents a video capture device managed by the platform.

| Attribute              | Data Type     | Nullable | Default      | Description                                    |
|------------------------|---------------|----------|--------------|------------------------------------------------|
| id                     | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| site_id                | UUID          | No       | —            | FK to Sites                                    |
| camera_group_id        | UUID          | Yes      | NULL         | FK to Camera Groups                            |
| name                   | VARCHAR(128)  | No       | —            | Camera display name                            |
| stream_url             | TEXT          | No       | —            | Primary stream URL                             |
| rtsp_url               | TEXT          | Yes      | NULL         | RTSP stream URL                                |
| status                 | ENUM          | No       | 'active'     | Camera status: active, offline, decommissioned |
| fps                    | INTEGER       | No       | 30           | Frames per second                              |
| resolution_width       | INTEGER       | No       | 1920         | Horizontal resolution in pixels                |
| resolution_height      | INTEGER       | No       | 1080         | Vertical resolution in pixels                  |
| night_vision_enabled   | BOOLEAN       | No       | false        | Night vision capability flag                   |
| motion_detection_enabled| BOOLEAN      | No       | true         | Motion detection enabled flag                  |
| storage_mode           | ENUM          | No       | 'cloud'      | Storage: cloud, local, hybrid                  |
| metadata               | JSONB         | Yes      | '{}'         | Additional camera metadata                     |
| last_health_check      | TIMESTAMP     | Yes      | NULL         | Last health check timestamp                    |
| created_at             | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |
| updated_at             | TIMESTAMP     | No       | NOW()        | Last modification timestamp                    |
| deleted_at             | TIMESTAMP     | Yes      | NULL         | Soft deletion timestamp                        |

**Constraints:**
- FK(site_id) REFERENCES Sites(id) ON DELETE RESTRICT
- FK(camera_group_id) REFERENCES Camera_Groups(id) ON DELETE SET NULL
- CHECK(status IN ('active', 'offline', 'decommissioned'))
- CHECK(fps > 0 AND fps <= 60)
- CHECK(resolution_width > 0 AND resolution_width <= 7680)
- CHECK(resolution_height > 0 AND resolution_height <= 4320)
- CHECK(storage_mode IN ('cloud', 'local', 'hybrid'))

### 10.7 Detection Event Entity

The Detection Event entity represents an AI-generated detection of objects, persons, or behaviors from camera feeds.

| Attribute              | Data Type     | Nullable | Default      | Description                                    |
|------------------------|---------------|----------|--------------|------------------------------------------------|
| id                     | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| camera_id              | UUID          | No       | —            | FK to Cameras                                  |
| rule_id                | UUID          | Yes      | NULL         | FK to Rules (if rule-triggered)                |
| event_type             | VARCHAR(64)   | No       | —            | Detection type: person, vehicle, object, behavior |
| severity               | ENUM          | No       | 'low'        | Event severity: low, medium, high, critical   |
| confidence_score       | DECIMAL(5,4)  | No       | —            | AI confidence score (0.0000 to 1.0000)        |
| detected_objects       | JSONB         | Yes      | '[]'         | Array of detected object metadata              |
| bounding_boxes         | JSONB         | Yes      | '[]'         | Array of bounding box coordinates              |
| zone_id                | VARCHAR(64)   | Yes      | NULL         | Monitoring zone identifier                     |
| metadata               | JSONB         | Yes      | '{}'         | Additional event metadata                      |
| processing_status      | ENUM          | No       | 'processed'  | Processing: pending, processed, failed        |
| detected_at            | TIMESTAMP     | No       | —            | Timestamp when detection occurred              |
| processed_at           | TIMESTAMP     | Yes      | NULL         | Timestamp when event was processed             |
| created_at             | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |

**Constraints:**
- FK(camera_id) REFERENCES Cameras(id) ON DELETE RESTRICT
- CHECK(severity IN ('low', 'medium', 'high', 'critical'))
- CHECK(confidence_score >= 0 AND confidence_score <= 1)
- CHECK(processing_status IN ('pending', 'processed', 'failed'))

### 10.8 Rule Entity

The Rule entity defines conditional logic that triggers alerts based on detection criteria.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| site_id            | UUID          | No       | —            | FK to Sites                                    |
| name               | VARCHAR(128)  | No       | —            | Rule name                                      |
| description        | TEXT          | Yes      | NULL         | Rule description                               |
| rule_type          | VARCHAR(64)   | No       | —            | Rule type: zone, schedule, composite          |
| conditions         | JSONB         | No       | —            | Rule condition definitions                     |
| actions            | JSONB         | No       | —            | Actions triggered by rule match                |
| severity           | ENUM          | No       | 'medium'     | Alert severity when rule matches               |
| is_enabled         | BOOLEAN       | No       | true         | Whether rule is active                         |
| priority           | INTEGER       | No       | 100          | Rule evaluation priority (lower = higher priority) |
| created_at         | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |
| updated_at         | TIMESTAMP     | No       | NOW()        | Last modification timestamp                    |
| deleted_at         | TIMESTAMP     | Yes      | NULL         | Soft deletion timestamp                        |

**Constraints:**
- FK(site_id) REFERENCES Sites(id) ON DELETE RESTRICT
- CHECK(severity IN ('low', 'medium', 'high', 'critical'))
- CHECK(priority >= 1 AND priority <= 1000)

### 10.9 Incident Entity

The Incident entity represents an operational response to a significant security event.

| Attribute              | Data Type     | Nullable | Default      | Description                                    |
|------------------------|---------------|----------|--------------|------------------------------------------------|
| id                     | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| detection_event_id     | UUID          | Yes      | NULL         | FK to Detection Events                         |
| assigned_user_id       | UUID          | Yes      | NULL         | FK to Users (assigned operator)                |
| site_id                | UUID          | No       | —            | FK to Sites                                    |
| title                  | VARCHAR(255)  | No       | —            | Incident title                                 |
| description            | TEXT          | Yes      | NULL         | Detailed incident description                  |
| severity               | ENUM          | No       | 'medium'     | Incident severity: low, medium, high, critical|
| status                 | ENUM          | No       | 'open'       | Status: open, investigating, resolved, closed |
| priority               | ENUM          | No       | 'medium'     | Priority: low, medium, high, urgent           |
| metadata               | JSONB         | Yes      | '{}'         | Additional incident metadata                   |
| acknowledged_at        | TIMESTAMP     | Yes      | NULL         | When incident was acknowledged                 |
| resolved_at            | TIMESTAMP     | Yes      | NULL         | When incident was resolved                     |
| created_at             | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |
| updated_at             | TIMESTAMP     | No       | NOW()        | Last modification timestamp                    |

**Constraints:**
- FK(detection_event_id) REFERENCES Detection_Events(id) ON DELETE SET NULL
- FK(assigned_user_id) REFERENCES Users(id) ON DELETE SET NULL
- FK(site_id) REFERENCES Sites(id) ON DELETE RESTRICT
- CHECK(severity IN ('low', 'medium', 'high', 'critical'))
- CHECK(status IN ('open', 'investigating', 'resolved', 'closed'))
- CHECK(priority IN ('low', 'medium', 'high', 'urgent'))

### 10.10 Evidence Entity

The Evidence entity represents video clips, images, or documents supporting an incident investigation.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| incident_id        | UUID          | No       | —            | FK to Incidents                                |
| camera_id          | UUID          | No       | —            | FK to Cameras                                  |
| evidence_type      | ENUM          | No       | —            | Type: video_clip, image, audio, document      |
| file_path          | TEXT          | No       | —            | Storage path to evidence file                  |
| file_name          | VARCHAR(255)  | No       | —            | Original file name                             |
| mime_type          | VARCHAR(128)  | No       | —            | MIME type of evidence file                     |
| file_size_bytes    | BIGINT        | No       | —            | File size in bytes                             |
| storage_bucket     | VARCHAR(128)  | No       | —            | S3 bucket or storage container                 |
| storage_key        | TEXT          | No       | —            | Object key in storage bucket                   |
| metadata           | JSONB         | Yes      | '{}'         | Additional evidence metadata                   |
| captured_at        | TIMESTAMP     | No       | —            | When evidence was captured                     |
| created_at         | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |
| deleted_at         | TIMESTAMP     | Yes      | NULL         | Soft deletion timestamp                        |

**Constraints:**
- FK(incident_id) REFERENCES Incidents(id) ON DELETE CASCADE
- FK(camera_id) REFERENCES Cameras(id) ON DELETE RESTRICT
- CHECK(evidence_type IN ('video_clip', 'image', 'audio', 'document'))
- CHECK(file_size_bytes > 0)

### 10.11 Evidence Hash Entity

The Evidence Hash entity stores cryptographic hashes ensuring evidence integrity and chain of custody.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| evidence_id        | UUID          | No       | —            | FK to Evidence                                 |
| hash_algorithm     | VARCHAR(32)   | No       | 'SHA-256'    | Hash algorithm used                            |
| hash_value         | VARCHAR(128)  | No       | —            | Computed hash value                            |
| signature          | TEXT          | Yes      | NULL         | Digital signature (optional)                   |
| computed_at        | TIMESTAMP     | No       | NOW()        | When hash was computed                         |
| verified_at        | TIMESTAMP     | Yes      | NULL         | When hash was last verified                    |

**Constraints:**
- FK(evidence_id) REFERENCES Evidence(id) ON DELETE CASCADE
- UNIQUE(evidence_id, hash_algorithm)
- CHECK(hash_algorithm IN ('SHA-256', 'SHA-512', 'MD5'))

### 10.12 Audit Log Entity

The Audit Log entity provides an immutable record of all significant system actions.

| Attribute          | Data Type     | Nullable | Default      | Description                                    |
|--------------------|---------------|----------|--------------|------------------------------------------------|
| id                 | UUID          | No       | gen_random_uuid() | Unique identifier                     |
| user_id            | UUID          | Yes      | NULL         | FK to Users (NULL for system actions)          |
| entity_type        | VARCHAR(64)   | No       | —            | Type of entity modified                        |
| entity_id          | UUID          | No       | —            | ID of entity modified                          |
| action             | VARCHAR(32)   | No       | —            | Action performed: create, update, delete, login |
| old_values         | JSONB         | Yes      | NULL         | Previous state (for updates)                   |
| new_values         | JSONB         | Yes      | NULL         | New state (for creates/updates)                |
| ip_address         | INET          | Yes      | NULL         | Client IP address                              |
| user_agent         | TEXT          | Yes      | NULL         | Client user agent string                       |
| created_at         | TIMESTAMP     | No       | NOW()        | Record creation timestamp                      |

**Constraints:**
- FK(user_id) REFERENCES Users(id) ON DELETE SET NULL
- CHECK(action IN ('create', 'update', 'delete', 'login', 'logout', 'export', 'access'))
- CHECK(entity_type IN ('user', 'role', 'permission', 'site', 'camera', 'camera_group', 'rule', 'incident', 'evidence', 'report', 'configuration', 'notification_rule'))

---

## 11. Physical Data Model

### 11.1 Storage Engine Configuration

#### SQLite Physical Storage (MVP)

| Parameter                    | Value                           | Rationale                          |
|------------------------------|---------------------------------|------------------------------------|
| Page Size                    | 4096 bytes                      | Optimal for most read/write patterns |
| Journal Mode                 | WAL (Write-Ahead Logging)       | Concurrent read/write support      |
| Synchronous                  | NORMAL                          | Balance of safety and performance  |
| Cache Size                   | -64000 (64 MB)                  | Adequate for MVP workload          |
| Foreign Keys                 | ON                              | Enforce referential integrity      |
| Busy Timeout                 | 5000 ms                         | Handle concurrent access gracefully|
| Auto Vacuum                  | INCREMENTAL                     | Reclaim space without full rebuild |
| Temp Store                   | MEMORY                          | Fast temporary table operations    |
| Secure Delete                | OFF                             | Performance optimization           |

#### PostgreSQL Physical Storage (Production)

| Parameter                    | Value                           | Rationale                          |
|------------------------------|---------------------------------|------------------------------------|
| Shared Buffers               | 25% of RAM                      | PostgreSQL recommendation          |
| Effective Cache Size         | 75% of RAM                      | Query planner optimization         |
| Work Memory                  | 256 MB                          | Complex query operations           |
| Maintenance Work Memory      | 2 GB                            | VACUUM, CREATE INDEX operations    |
| Max Connections              | 200                             | With PgBouncer connection pooling  |
| WAL Level                    | REPLICATE                       | Enable streaming replication       |
| Max WAL Senders              | 10                              | Support multiple replicas          |
| Checkpoint Completion Target | 0.9                             | Spread I/O during checkpoints      |
| Random Page Cost             | 1.1                             | SSD-optimized storage              |
| Effective IO Concurrency     | 200                             | SSD-optimized storage              |

### 11.2 Tablespace Configuration

#### SQLite Tablespace Strategy

SQLite uses a single file-based storage model with WAL journaling:

| Tablespace          | Purpose                              | Location                 |
|---------------------|--------------------------------------|--------------------------|
| Primary Data File   | All tables and indexes               | /data/vigilant.db        |
| WAL File            | Write-ahead log                      | /data/vigilant.db-wal    |
| SHM File            | Shared memory for WAL mode           | /data/vigilant.db-shm    |
| Temporary Storage   | Query execution temp data            | System temp directory    |

#### PostgreSQL Tablespace Strategy

| Tablespace          | Purpose                              | Storage Type     | IOPS Target |
|---------------------|--------------------------------------|------------------|-------------|
| pg_default          | Standard tables and indexes          | SSD              | 10,000      |
| pg_fast             | High-performance tables (hot data)   | NVMe SSD         | 50,000      |
| pg_archive          | Archived and historical data         | HDD / Cold SSD   | 1,000       |
| pg_temp             | Query execution temporary data       | RAM-backed SSD   | 100,000     |

### 11.3 Data Type Mapping

| Logical Type      | SQLite Type        | PostgreSQL Type      | Notes                              |
|-------------------|--------------------|----------------------|------------------------------------|
| UUID              | TEXT               | UUID                 | Stored as text in SQLite           |
| VARCHAR(n)        | TEXT               | VARCHAR(n)           | SQLite uses flexible typing        |
| TEXT               | TEXT               | TEXT                 | Direct mapping                     |
| BOOLEAN            | INTEGER (0/1)      | BOOLEAN              | SQLite uses integer for booleans   |
| INTEGER            | INTEGER            | INTEGER              | Direct mapping                     |
| BIGINT             | INTEGER            | BIGINT               | SQLite integer is variable-width   |
| DECIMAL(p,s)       | TEXT               | NUMERIC(p,s)         | Stored as text in SQLite for precision |
| TIMESTAMP          | TEXT (ISO 8601)    | TIMESTAMPTZ          | ISO 8601 format in SQLite          |
| JSONB              | TEXT (JSON)        | JSONB                | Validated JSON text in SQLite      |
| INET               | TEXT               | INET                 | Stored as text in SQLite           |
| ENUM               | TEXT               | VARCHAR + CHECK      | CHECK constraint in SQLite         |
| BYTEA              | BLOB               | BYTEA                | Binary data storage                |

### 11.4 Index Physical Organization

#### SQLite Index Organization

| Index Type          | Storage Structure | Use Case                          |
|---------------------|-------------------|-----------------------------------|
| Primary Key         | B-Tree (INTEGER)  | Auto-incrementing integer keys    |
| UNIQUE              | B-Tree            | Unique constraints                |
| Standard            | B-Tree            | General-purpose indexing          |
| FTS5                | Inverted Index    | Full-text search on evidence      |
| Partial             | B-Tree + WHERE    | Filtered indexes for active data  |

#### PostgreSQL Index Organization

| Index Type          | Storage Structure | Use Case                          |
|---------------------|-------------------|-----------------------------------|
| B-Tree              | Balanced Tree     | Equality and range queries        |
| Hash                | Hash Table        | Equality-only lookups             |
| GIN                 | Inverted Index    | JSONB, array, full-text search    |
| GiST                | Generalized Tree  | Geospatial, range types           |
| BRIN                | Block Range       | Time-series data, large tables    |
| Partial             | B-Tree + WHERE    | Filtered indexes for active data  |
| Expression          | B-Tree + Function | Indexed computed values           |

---

## 12. Data Dictionary

### 12.1 Naming Conventions

| Element               | Convention                          | Example                    |
|-----------------------|-------------------------------------|----------------------------|
| Table Names           | snake_case, plural                  | detection_events           |
| Column Names          | snake_case, singular                | created_at                 |
| Primary Key           | id                                  | id                         |
| Foreign Key           | {referenced_table_singular}_id      | camera_id                  |
| Index Names           | idx_{table}_{columns}               | idx_detection_events_camera_id |
| Unique Constraint     | uk_{table}_{columns}                | uk_users_username          |
| Check Constraint      | ck_{table}_{rule}                   | ck_users_status            |
| Foreign Key Constraint| fk_{table}_{referenced}             | fk_detection_events_cameras|
| Enum Values           | snake_case                          | 'video_clip'               |

### 12.2 Standard Columns

| Column              | Type        | Description                                        |
|---------------------|-------------|----------------------------------------------------|
| id                  | UUID        | Surrogate primary key for all entities             |
| created_at          | TIMESTAMP   | Record creation timestamp (auto-populated)         |
| updated_at          | TIMESTAMP   | Last modification timestamp (auto-updated)         |
| deleted_at          | TIMESTAMP   | Soft deletion timestamp (NULL = active)            |

---

## 13. Normalization and Denormalization Strategy

### 13.1 Normalization Approach

The VigilantAI data model follows Third Normal Form (3NF) as the baseline normalization level. This ensures minimal data redundancy while maintaining query performance and data integrity.

**First Normal Form (1NF):** All attributes contain atomic values. No repeating groups or arrays. JSONB columns are used for semi-structured metadata that varies between entities.

**Second Normal Form (2NF):** All non-key attributes are fully functionally dependent on the primary key. Composite primary keys are avoided in favor of surrogate UUID keys.

**Third Normal Form (3NF):** No transitive dependencies exist. All non-key attributes depend only on the primary key, not on other non-key attributes.

### 13.2 Strategic Denormalization

While 3NF is the baseline, select strategic denormalization is employed for performance-critical query patterns:

| Denormalized Element             | Location                | Justification                                      |
|----------------------------------|-------------------------|-----------------------------------------------------|
| camera.name in detection_events  | Query results (views)   | Avoid JOINs for high-volume event display            |
| site.name in camera records      | Camera table            | Reduce JOINs for camera listing queries              |
| user.display_name in audit_logs  | Audit log table         | Avoid JOINs for audit trail display                  |
| Latest health status in cameras  | Camera table (cached)   | Eliminate JOINs for dashboard camera status          |
| Aggregate counts in reports      | Materialized views      | Pre-computed analytics for dashboard performance     |

### 13.3 Materialized Views

Materialized views are used for pre-computed aggregations that support dashboard and reporting requirements:

**Daily Camera Health Summary:**
Aggregates camera health metrics by day, site, and camera group for dashboard display and trend analysis.

**Incident Response Metrics:**
Pre-computes incident counts, response times, and resolution rates by severity, site, and time period.

**Detection Event Statistics:**
Aggregates detection event volumes, types, and confidence distributions by camera, zone, and time period.

**Notification Delivery Metrics:**
Computes notification success rates, delivery times, and failure patterns by channel and recipient.

---

## 14. Data Lifecycle Management

### 14.1 Entity Lifecycle States

| Entity            | States                                    | Retention Policy                              |
|-------------------|-------------------------------------------|-----------------------------------------------|
| User              | Active → Inactive → Deleted               | Active: Indefinite; Deleted: 90 days (soft)   |
| Session           | Active → Expired                          | Expired: 30 days                              |
| API Key           | Active → Revoked                          | Revoked: 30 days                              |
| Camera            | Active → Offline → Decommissioned         | Decommissioned: 1 year                        |
| Detection Event   | New → Processed → Archived → Purged       | Archived: 90 days; Purged: 1 year             |
| Incident          | Open → Investigating → Resolved → Closed  | Closed: 2 years                               |
| Evidence          | Captured → Verified → Archived → Purged   | Archived: 1 year; Legal hold: Indefinite      |
| Audit Log         | Created → Archived → Purged               | Archived: 1 year; Purged: 7 years             |
| Notification      | Sent → Delivered → Failed                 | Failed: 30 days; Delivered: 90 days           |
| Report            | Draft → Published                         | Published: 1 year                             |
| Camera Health     | Current → Historical                      | Historical: 30 days (raw); 1 year (aggregated)|
| Health Metric     | Current → Historical                      | Historical: 7 days                            |
| System Config     | Active → Updated                          | Updated: retains all versions                 |

### 14.2 Data Archival Strategy

**Hot Data (0–30 days):** Active, frequently accessed data stored in primary tables with full indexing. Optimized for read and write performance.

**Warm Data (30–90 days):** Recently archived data stored in partitioned tables with reduced indexing. Accessible but not optimized for frequent queries.

**Cold Data (90+ days):** Historical data archived to separate tablespaces or cloud storage. Accessed only for compliance, investigation, or long-term analytics.

**Legal Hold:** Data subject to legal proceedings or regulatory investigation is exempt from standard retention policies and archived indefinitely until hold is released by Legal Counsel.

### 14.3 Automated Cleanup Procedures

| Cleanup Task                       | Frequency    | Trigger                       | Action                            |
|------------------------------------|--------------|-------------------------------|-----------------------------------|
| Expired session removal            | Hourly       | Scheduled job                 | DELETE WHERE expires_at < NOW()   |
| Soft-deleted entity hard delete    | Daily        | Scheduled job                 | DELETE WHERE deleted_at < NOW() - interval '90 days' |
| Detection event archival           | Daily        | Scheduled job                 | Move to archive partition         |
| Detection event purge              | Weekly       | Scheduled job                 | DELETE from archive WHERE age > 1 year |
| Audit log archival                 | Monthly      | Scheduled job                 | Move to cold storage              |
| Notification history cleanup       | Daily        | Scheduled job                 | DELETE WHERE age > 90 days (delivered) |
| Camera health data aggregation     | Hourly       | Scheduled job                 | Aggregate and store in summary table |
| Health metric cleanup              | Daily        | Scheduled job                 | DELETE WHERE age > 7 days         |
| Report cleanup                     | Monthly      | Scheduled job                 | DELETE WHERE age > 1 year         |
| API key revocation check           | Hourly       | Scheduled job                 | Mark expired keys as revoked      |

---

## 15. Indexing Strategy

### 15.1 Index Design Principles

1. **Query-Driven:** Every index must be justified by a specific, identified query pattern. No speculative indexing.
2. **Selective Indexing:** Prefer indexes on high-selectivity columns (many distinct values) over low-selectivity columns.
3. **Composite Index Order:** In composite indexes, place equality conditions before range conditions, and high-selectivity columns first.
4. **Partial Indexes:** Use partial indexes (WHERE clause) to index only active or relevant subsets of data.
5. **Covering Indexes:** Where possible, include all query-required columns in the index to avoid table lookups.
6. **Minimal Index Footprint:** Balance query performance against storage and write overhead.

### 15.2 Required Indexes

#### Identity & Access Domain

| Index Name                          | Table           | Columns                    | Type       | Justification                          |
|-------------------------------------|-----------------|----------------------------|------------|----------------------------------------|
| idx_users_username                  | users           | username                   | B-Tree/Unique | Login lookup by username            |
| idx_users_email                     | users           | email                      | B-Tree/Unique | Login lookup by email               |
| idx_users_status                    | users           | status                     | B-Tree     | Active user queries                    |
| idx_user_sessions_user_id           | user_sessions   | user_id                    | B-Tree     | User session listing                   |
| idx_user_sessions_expires_at        | user_sessions   | expires_at                 | B-Tree     | Session expiration cleanup             |
| idx_user_roles_user_id              | user_roles      | user_id                    | B-Tree     | User role lookup                       |
| idx_user_roles_role_id              | user_roles      | role_id                    | B-Tree     | Role member listing                    |
| idx_api_keys_user_id                | api_keys        | user_id                    | B-Tree     | API key listing per user               |

#### Physical Assets Domain

| Index Name                          | Table           | Columns                    | Type       | Justification                          |
|-------------------------------------|-----------------|----------------------------|------------|----------------------------------------|
| idx_cameras_site_id                 | cameras         | site_id                    | B-Tree     | Camera listing per site                |
| idx_cameras_camera_group_id         | cameras         | camera_group_id            | B-Tree     | Camera listing per group               |
| idx_cameras_status                  | cameras         | status                     | B-Tree     | Active camera queries                  |
| idx_camera_health_camera_id         | camera_health   | camera_id, recorded_at     | B-Tree     | Health history per camera              |
| idx_camera_health_recorded_at       | camera_health   | recorded_at                | BRIN       | Time-series health data (PostgreSQL)  |
| idx_camera_groups_site_id           | camera_groups   | site_id                    | B-Tree     | Group listing per site                 |

#### Event Processing Domain

| Index Name                          | Table           | Columns                    | Type       | Justification                          |
|-------------------------------------|-----------------|----------------------------|------------|----------------------------------------|
| idx_detection_events_camera_id      | detection_events| camera_id                  | B-Tree     | Events per camera                      |
| idx_detection_events_detected_at    | detection_events| detected_at                | BRIN       | Time-series event data (PostgreSQL)   |
| idx_detection_events_severity       | detection_events| severity                   | B-Tree     | Severity-based filtering               |
| idx_detection_events_status         | detection_events| processing_status          | B-Tree     | Processing queue queries               |
| idx_detection_events_rule_id        | detection_events| rule_id                    | B-Tree     | Events triggered by specific rule      |
| idx_rules_site_id                   | rules           | site_id                    | B-Tree     | Rules per site                         |
| idx_rules_enabled                   | rules           | is_enabled                 | B-Tree     | Active rule queries                    |
| idx_alerts_detection_event_id       | alerts          | detection_event_id         | B-Tree     | Alerts for specific event              |
| idx_alerts_status                   | alerts          | status                     | B-Tree     | Pending alert queries                  |

#### Response Management Domain

| Index Name                          | Table           | Columns                    | Type       | Justification                          |
|-------------------------------------|-----------------|----------------------------|------------|----------------------------------------|
| idx_incidents_site_id               | incidents       | site_id                    | B-Tree     | Incidents per site                     |
| idx_incidents_status                | incidents       | status                     | B-Tree     | Open incident queries                  |
| idx_incidents_assigned_user_id      | incidents       | assigned_user_id           | B-Tree     | Incidents per operator                 |
| idx_incidents_created_at            | incidents       | created_at                 | B-Tree     | Incident timeline queries              |
| idx_evidence_incident_id            | evidence        | incident_id                | B-Tree     | Evidence per incident                  |
| idx_evidence_camera_id              | evidence        | camera_id                  | B-Tree     | Evidence per camera                    |
| idx_evidence_captured_at            | evidence        | captured_at                | B-Tree     | Evidence timeline queries              |

#### System & Operations Domain

| Index Name                          | Table           | Columns                    | Type       | Justification                          |
|-------------------------------------|-----------------|----------------------------|------------|----------------------------------------|
| idx_audit_logs_user_id              | audit_logs      | user_id                    | B-Tree     | Audit trail per user                   |
| idx_audit_logs_entity               | audit_logs      | entity_type, entity_id     | B-Tree     | Audit trail per entity                 |
| idx_audit_logs_created_at           | audit_logs      | created_at                 | BRIN       | Time-series audit data (PostgreSQL)   |
| idx_audit_logs_action               | audit_logs      | action                     | B-Tree     | Action-based filtering                 |
| idx_notification_history_user_id    | notification_history | user_id                | B-Tree     | Notifications per user                 |
| idx_notification_history_status     | notification_history | status                | B-Tree     | Failed notification queries            |
| idx_notification_history_sent_at    | notification_history | sent_at               | BRIN       | Time-series notification data (PostgreSQL) |

### 15.3 Index Maintenance

**PostgreSQL VACUUM Strategy:**
- autovacuum_enabled = on
- autovacuum_vacuum_scale_factor = 0.1 (10% dead tuples triggers VACUUM)
- autovacuum_analyze_scale_factor = 0.05 (5% changes triggers ANALYZE)
- autovacuum_vacuum_cost_delay = 2 (ms, reduces I/O impact)
- Manual VACUUM ANALYZE after bulk data loads

**SQLite Maintenance:**
- VACUUM during low-usage periods
- PRAGMA optimize after schema changes
- Regular integrity_check validation

---

## 16. Partitioning Strategy

### 16.1 Partitioning Approach

PostgreSQL declarative table partitioning is employed for high-volume, time-series data to improve query performance, simplify data lifecycle management, and enable efficient archival and purging.

### 16.2 Partitioned Tables

| Table                | Partition Key    | Strategy     | Partition Interval | Retention         |
|----------------------|------------------|--------------|--------------------|--------------------|
| detection_events     | detected_at      | Range        | Monthly            | 12 months          |
| audit_logs           | created_at       | Range        | Monthly            | 84 months (7 years)|
| camera_health        | recorded_at      | Range        | Weekly             | 12 months          |
| notification_history | sent_at          | Range        | Monthly            | 6 months           |
| health_metrics       | recorded_at      | Range        | Daily              | 7 days             |

### 16.3 Partition Pruning

Query performance is optimized through partition pruning, where the query planner automatically excludes partitions that cannot contain relevant data based on the partition key. This is particularly effective for time-range queries that are common in surveillance analytics:

- "Show me detection events from the last 24 hours" → scans only the current monthly partition
- "Export all audit logs from last quarter" → scans only the relevant monthly partitions
- "View camera health history for this week" → scans only the current weekly partition

### 16.4 SQLite Partitioning Alternative

SQLite does not support native table partitioning. The following strategies are employed:

**Logical Partitioning:** Application-level date-based table naming (e.g., detection_events_2026_01, detection_events_2026_02). Application code queries the appropriate table based on the requested time range.

**View-Based Abstraction:** Database views combine logically partitioned tables, providing a unified query interface to application code.

**Automated Table Management:** Scheduled jobs create new monthly tables and drop expired tables according to the retention policy.

---

## 17. Replication and High Availability

### 17.1 SQLite High Availability (MVP)

SQLite operates in single-server mode. High availability is achieved through:

| Strategy                    | Implementation                                        |
|-----------------------------|-------------------------------------------------------|
| Regular Backups             | File-level copy with WAL checkpoint before backup      |
| WAL Shipping               | rsync WAL files to standby server                      |
| Application-Level Sync     | Periodic database synchronization between servers      |
| Filesystem Replication     | ZFS/Btrfs snapshot replication                         |

### 17.2 PostgreSQL High Availability (Production)

| Strategy                    | Implementation                                        |
|-----------------------------|-------------------------------------------------------|
| Streaming Replication       | Synchronous/asynchronous replication to hot standby    |
| Logical Replication         | Selective table replication for read scaling           |
| Automatic Failover          | Patroni or pg_auto_failover for leader election        |
| Load Balancing              | PgBouncer + HAProxy for read traffic distribution     |
| Connection Pooling          | PgBouncer with transaction-level pooling               |

### 17.3 Replication Topology

```
┌─────────────────────────────────────────────────────────────────┐
│                    PostgreSQL Replication Topology               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                    ┌──────────────┐                             │
│                    │   Primary    │                             │
│                    │  (Read/Write)│                             │
│                    └──────┬───────┘                             │
│                           │                                     │
│              ┌────────────┼────────────┐                        │
│              ▼            ▼            ▼                        │
│     ┌──────────────┐ ┌──────────────┐ ┌──────────────┐         │
│     │   Standby 1  │ │   Standby 2  │ │   Standby 3  │         │
│     │  (Hot)       │ │  (Hot)       │ │  (Warm)      │         │
│     │  (Read Only) │ │  (Read Only) │ │  (Read Only) │         │
│     └──────────────┘ └──────────────┘ └──────────────┘         │
│                                                                 │
│     Streaming Replication (sync)  │  Streaming Replication      │
│                                   │  (async, remote site)       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 17.4 Consistency Model

**Synchronous Replication:** Critical data (incidents, evidence, audit logs) is replicated synchronously to ensure zero data loss (RPO = 0).

**Asynchronous Replication:** Non-critical data (health metrics, detection event details) is replicated asynchronously to minimize latency impact.

**Conflict Resolution:** Leader election via Patroni/etcd ensures single-writer topology. No write-write conflicts occur in normal operation.

---

## 18. Backup and Recovery

### 18.1 Backup Strategy

| Backup Type           | Frequency      | Retention    | Storage Location             |
|-----------------------|----------------|--------------|------------------------------|
| Full Database Backup  | Daily 02:00 UTC| 30 days      | S3 (Standard) + Local Disk   |
| Incremental Backup    | Hourly         | 7 days       | S3 (Standard)                |
| WAL Archival          | Continuous     | 14 days      | S3 (Standard)                |
| Logical Export (pg_dump)| Weekly        | 90 days      | S3 (Glacier)                 |
| SQLite File Copy      | Hourly         | 7 days       | Local + Network Share        |
| Snapshot Backup       | On-demand      | 30 days      | EBS Snapshots                |

### 18.2 Recovery Objectives

| Metric                    | SQLite (MVP)    | PostgreSQL (Production) |
|---------------------------|-----------------|-------------------------|
| Recovery Time Objective   | 4 hours         | 1 hour                  |
| Recovery Point Objective  | 1 hour          | 0 (zero data loss)      |
| Maximum Tolerable Downtime| 8 hours         | 2 hours                 |
| Backup Verification       | Weekly          | Daily                   |

### 18.3 Recovery Procedures

**Point-in-Time Recovery (PostgreSQL):**
1. Identify the target recovery time
2. Restore the most recent full backup
3. Apply WAL archives sequentially up to the target time
4. Verify data integrity
5. Promote standby if primary is unrecoverable
6. Update DNS/connection strings if needed

**SQLite Recovery:**
1. Identify the most recent valid backup
2. Copy backup file to production location
3. Apply WAL file if available and consistent
4. Verify database integrity using PRAGMA integrity_check
5. Restart application services

---

## 19. Data Security and Encryption

### 19.1 Encryption at Rest

| Data Category           | Algorithm       | Key Management              | Scope                  |
|-------------------------|-----------------|-----------------------------|------------------------|
| Authentication Data     | AES-256-GCM     | AWS KMS / HSM               | Password hashes, MFA secrets, API keys |
| Evidence Files          | AES-256-CBC     | AWS KMS                     | Video clips, images, documents |
| Audit Logs              | AES-256-GCM     | AWS KMS                     | All audit trail data   |
| Database Files          | SQLCipher (AES-256) | Passphrase (derived from KMS) | Entire SQLite database |
| Tablespaces             | TDE (PostgreSQL)| AWS KMS                     | PostgreSQL tablespaces |
| Backups                 | AES-256-GCM     | Separate backup key         | All backup files       |

### 19.2 Encryption in Transit

| Connection Type                | Protocol        | Certificate Type           | Minimum Version |
|--------------------------------|-----------------|----------------------------|-----------------|
| Client → API Server           | TLS             | Let's Encrypt / AWS ACM    | TLS 1.2         |
| API Server → Database         | TLS             | Private CA (internal)      | TLS 1.3         |
| Database → Replica            | TLS             | Private CA (internal)      | TLS 1.3         |
| Client → MinIO/S3             | TLS             | Private CA (internal)      | TLS 1.2         |
| WebSocket Connections         | TLS (WSS)       | Same as API Server         | TLS 1.2         |

### 19.3 Key Rotation Policy

| Key Type                    | Rotation Frequency | Rotation Procedure                              |
|-----------------------------|--------------------|-----------------|
| Database Encryption Key     | 90 days            | KMS automatic rotation                           |
| Evidence Storage Key        | 90 days            | KMS automatic rotation                           |
| Backup Encryption Key       | 180 days           | Manual rotation with dual-key transition         |
| JWT Signing Key             | 30 days            | Automated with grace period for existing tokens  |
| TLS Certificates            | 90 days            | Auto-renewal via cert-manager / ACM              |
| API Key Hashing Salt        | 365 days           | Manual rotation with rehashing of existing keys  |

### 19.4 Access Control

**Database User Roles:**

| Role                      | Permissions                                   | Usage                     |
|---------------------------|-----------------------------------------------|---------------------------|
| vigilant_app              | SELECT, INSERT, UPDATE, DELETE on all tables  | Application read/write    |
| vigilant_readonly         | SELECT on all tables                          | Reporting and analytics   |
| vigilant_admin            | DDL + DML on all tables                       | Schema management         |
| vigilant_backup           | SELECT + pg_dump                              | Backup operations         |
| vigilant_monitor          | pg_stat_* views                              | Monitoring and observability |

**Row-Level Security (PostgreSQL):**
Row-Level Security policies enforce data isolation at the database level, ensuring that users can only access data for sites they are authorized to view. This provides defense-in-depth beyond application-layer authorization.

### 19.5 Data Masking

Sensitive data is masked in non-production environments:

| Data Field               | Masking Technique                              |
|--------------------------|------------------------------------------------|
| Password Hashes          | Not replicated to non-production               |
| MFA Secrets              | Random replacement                             |
| API Key Hashes           | Random replacement                             |
| Email Addresses          | Partial redaction (j***@example.com)           |
| Phone Numbers            | Partial redaction (***-***-1234)               |
| IP Addresses             | Last octet zeroed (192.168.1.0)                |

---

## 20. Data Validation and Integrity Constraints

### 20.1 Domain-Level Constraints

| Constraint Type          | Examples                                                    |
|--------------------------|-------------------------------------------------------------|
| CHECK Constraints        | Status values, numeric ranges, format validation            |
| UNIQUE Constraints       | Username, email, config keys, (evidence_id, hash_algorithm) |
| NOT NULL Constraints     | All primary keys, required business fields                  |
| DEFAULT Values           | Timestamps, status fields, JSONB metadata defaults          |

### 20.2 Referential Integrity Constraints

| Constraint                          | Action on Parent Delete | Action on Parent Update |
|-------------------------------------|-------------------------|-------------------------|
| FK(user_sessions → users)          | CASCADE                 | CASCADE                 |
| FK(api_keys → users)              | CASCADE                 | CASCADE                 |
| FK(user_roles → users)            | CASCADE                 | CASCADE                 |
| FK(user_roles → roles)            | CASCADE                 | CASCADE                 |
| FK(role_permissions → roles)      | CASCADE                 | CASCADE                 |
| FK(role_permissions → permissions)| CASCADE                 | CASCADE                 |
| FK(cameras → sites)               | RESTRICT                | CASCADE                 |
| FK(cameras → camera_groups)       | SET NULL                | CASCADE                 |
| FK(camera_health → cameras)       | CASCADE                 | CASCADE                 |
| FK(detection_events → cameras)    | RESTRICT                | CASCADE                 |
| FK(rules → sites)                 | RESTRICT                | CASCADE                 |
| FK(alerts → detection_events)     | CASCADE                 | CASCADE                 |
| FK(alerts → rules)               | SET NULL                | CASCADE                 |
| FK(incidents → detection_events)  | SET NULL                | CASCADE                 |
| FK(incidents → users)             | SET NULL                | CASCADE                 |
| FK(incidents → sites)             | RESTRICT                | CASCADE                 |
| FK(evidence → incidents)          | CASCADE                 | CASCADE                 |
| FK(evidence → cameras)            | RESTRICT                | CASCADE                 |
| FK(evidence_hash → evidence)      | CASCADE                 | CASCADE                 |
| FK(notification_history → notification_rules)| SET NULL   | CASCADE                 |
| FK(notification_history → users) | SET NULL                | CASCADE                 |
| FK(audit_logs → users)            | SET NULL                | CASCADE                 |

### 20.3 Business Rule Constraints

| Rule ID  | Rule Description                                          | Enforcement Level |
|----------|-----------------------------------------------------------|-------------------|
| BR-DB-01 | Usernames must be 3–64 characters, alphanumeric + underscore | Database CHECK  |
| BR-DB-02 | Email addresses must match valid format                   | Database CHECK    |
| BR-DB-03 | Camera FPS must be between 1 and 60                       | Database CHECK    |
| BR-DB-04 | Detection event confidence must be between 0 and 1        | Database CHECK    |
| BR-DB-05 | Rule priority must be between 1 and 1000                  | Database CHECK    |
| BR-DB-06 | Evidence file size must be greater than 0                  | Database CHECK    |
| BR-DB-07 | GPS coordinates must be within valid ranges                | Database CHECK    |
| BR-DB-08 | Session expiry must be after session creation              | Database CHECK    |
| BR-DB-09 | Audit logs are immutable (no UPDATE or DELETE allowed)     | Database trigger  |
| BR-DB-10 | Evidence hash must be computed within 5 minutes of capture| Application layer |
| BR-DB-11 | Maximum 5 concurrent sessions per user                     | Application layer |
| BR-DB-12 | Incident severity must match or exceed detection event severity | Application layer |

---

## 21. Transaction Management and Concurrency Control

### 21.1 Transaction Isolation Levels

| Database     | Default Isolation Level | Configured Level         | Rationale                      |
|--------------|-------------------------|--------------------------|--------------------------------|
| SQLite       | SERIALIZABLE (WAL)      | SERIALIZABLE             | Only isolation level available |
| PostgreSQL   | READ COMMITTED           | READ COMMITTED           | Balance of consistency and performance |

### 21.2 Concurrency Control Mechanisms

**SQLite Concurrency:**
- WAL mode enables concurrent readers with a single writer
- Database-level locking (no row-level locking)
- Busy timeout of 5 seconds for handling lock contention
- Single-writer architecture limits write throughput

**PostgreSQL Concurrency:**
- MVCC (Multi-Version Concurrency Control) for non-blocking reads
- Row-level locking for write operations
- Advisory locks for application-level coordination
- SELECT FOR UPDATE for pessimistic locking when needed

### 21.3 Critical Transaction Patterns

**Incident Creation Transaction:**
1. Begin transaction
2. INSERT incident record
3. INSERT evidence records
4. INSERT evidence hash records
5. INSERT audit log entries
6. Commit transaction (all-or-nothing)

**Evidence Verification Transaction:**
1. Begin transaction (READ ONLY)
2. SELECT evidence file and hash
3. Compute hash of retrieved file
4. Compare with stored hash
5. Update verified_at timestamp
6. Commit transaction

### 21.4 Deadlock Prevention

| Strategy                        | Implementation                                    |
|---------------------------------|---------------------------------------------------|
| Consistent Lock Ordering        | Always lock tables in alphabetical order          |
| Lock Timeout                    | 5 seconds maximum wait for lock acquisition       |
| Statement Timeout               | 30 seconds maximum per statement                  |
| Deadlock Detection              | PostgreSQL automatic deadlock detection           |
| Application-Level Retry         | Exponential backoff with jitter on deadlock error |

---

## 22. Query Performance and Optimization

### 22.1 Expected Query Patterns

| Query Pattern                              | Frequency    | Latency Target | Optimization Strategy          |
|--------------------------------------------|--------------|----------------|--------------------------------|
| User authentication lookup                 | High         | < 10ms         | Index on username/email        |
| Camera listing per site                    | High         | < 50ms         | Index on site_id + status      |
| Detection event timeline query             | High         | < 100ms        | BRIN index on detected_at     |
| Incident listing with filters              | High         | < 100ms        | Composite index on status/site |
| Evidence retrieval by incident             | Medium       | < 100ms        | Index on incident_id          |
| Audit log search by entity                 | Medium       | < 200ms        | Composite index on entity     |
| Dashboard aggregate statistics             | High         | < 200ms        | Materialized views            |
| Camera health history (last 24h)           | Medium       | < 100ms        | BRIN index + partition pruning |
| Full-text evidence search                  | Low          | < 1000ms       | FTS5 (SQLite) / tsvector (PG) |
| Report generation (monthly aggregates)     | Low          | < 5000ms       | Materialized views            |

### 22.2 Query Optimization Guidelines

1. **Use EXPLAIN ANALYZE** to validate query plans before deployment
2. **Avoid SELECT *** in production queries; select only required columns
3. **Use parameterized queries** to enable plan caching (SQLx provides this)
4. **Prefer covering indexes** for frequently executed read queries
5. **Use LIMIT** to bound result sets in user-facing queries
6. **Avoid N+1 query patterns**; use JOINs or batch fetching
7. **Leverage partition pruning** by including partition key in WHERE clause
8. **Use prepared statements** for repeated query patterns

### 22.3 Performance Monitoring

| Metric                          | Warning Threshold | Critical Threshold |
|---------------------------------|-------------------|--------------------|
| Average Query Latency           | > 100ms           | > 500ms            |
| P95 Query Latency               | > 500ms           | > 2000ms           |
| Slow Query Count (per minute)   | > 5               | > 20               |
| Connection Pool Utilization     | > 80%             | > 95%              |
| Cache Hit Ratio                 | < 95%             | < 85%              |
| Dead Tuple Count                | > 100,000         | > 1,000,000        |
| Table Bloat                     | > 20%             | > 40%              |

---

## 23. Connection Pooling and Resource Management

### 23.1 Connection Pool Architecture

**SQLite (MVP):**
SQLite uses in-process connections with no external pooling. The application maintains a single write connection and multiple read connections through WAL mode.

| Parameter                | Value          |
|--------------------------|----------------|
| Max Read Connections     | 4              |
| Max Write Connections    | 1              |
| Connection Timeout       | 5 seconds      |
| Idle Timeout             | 300 seconds    |

**PostgreSQL (Production) with PgBouncer:**

| Parameter                | Value          | Rationale                        |
|--------------------------|----------------|----------------------------------|
| Pool Mode                | Transaction    | Optimal for short transactions   |
| Default Pool Size        | 20             | Per-user/database pair           |
| Max Client Connections   | 200            | Application-side limit           |
| Max DB Connections       | 50             | PostgreSQL-side limit            |
| Reserve Pool Size        | 5              | Burst capacity                   |
| Reserve Pool Timeout     | 3 seconds      | Wait before using reserve        |
| Server Idle Timeout      | 600 seconds    | Release idle connections         |
| Client Idle Timeout      | 300 seconds    | Disconnect idle clients          |
| Query Timeout            | 30 seconds     | Kill long-running queries        |

### 23.2 Connection Lifecycle

```
Application → PgBouncer (Transaction Pool) → PostgreSQL
              │                              │
              │ 1. Client connects           │
              │ 2. Transaction starts        │
              │ 3. Assign server connection  │ → Server connection assigned
              │ 4. Execute queries           │ → Queries executed
              │ 5. Transaction commits      │
              │ 6. Release server connection │ → Server connection released
              │ 7. Return to pool            │
```

### 23.3 Resource Limits

| Resource                        | SQLite Limit  | PostgreSQL Limit |
|---------------------------------|---------------|------------------|
| Max Open Files                  | 64            | Unlimited        |
| Max Memory Per Connection       | N/A           | 256 MB           |
| Max Query Execution Time        | 30 seconds    | 30 seconds       |
| Max Result Set Size             | 10,000 rows   | 10,000 rows      |
| Max Transaction Duration        | 60 seconds    | 60 seconds       |
| Max Prepared Statements         | 100           | Unlimited        |

---

## 24. Data Migration and Versioning

### 24.1 Schema Migration Strategy

The platform uses versioned schema migrations managed through SQLx's migration framework. Each migration is a versioned SQL file that can be applied or rolled back.

| Aspect                    | SQLite Approach           | PostgreSQL Approach         |
|---------------------------|---------------------------|------------------------------|
| Migration Tool            | SQLx migrate              | SQLx migrate                 |
| Version Tracking          | _sqlx_migrations table   | _sqlx_migrations table       |
| Rollback Support          | Manual (down migration)   | Manual (down migration)      |
| Schema Validation         | Compile-time (SQLx)       | Compile-time (SQLx)          |
| Data Migrations           | Separate migration files  | Separate migration files     |
| Branch Migrations         | Supported                 | Supported                    |

### 24.2 Migration File Structure

```
migrations/
├── sqlite/
│   ├── 001_create_users.sql
│   ├── 002_create_roles.sql
│   ├── 003_create_cameras.sql
│   └── ...
├── postgres/
│   ├── 001_create_users.sql
│   ├── 002_create_roles.sql
│   ├── 003_create_cameras.sql
│   └── ...
└── shared/
    ├── seed_roles.sql
    └── seed_permissions.sql
```

### 24.3 Migration Rules

1. **Forward-Only in Production:** Migrations are applied forward only; rollbacks require manual intervention and approval.
2. **Backward Compatible:** Schema changes must be backward compatible to support zero-downtime deployments.
3. **Additive Changes Preferred:** Prefer ADD COLUMN, ADD INDEX over ALTER COLUMN or DROP COLUMN.
4. **Data Migration Separation:** Schema changes and data migrations are separate migration files.
5. **Testing Required:** All migrations must be tested against both SQLite and PostgreSQL before deployment.
6. **Version Control:** Migration files are version-controlled and peer-reviewed like application code.
7. **Idempotent:** Migrations should be idempotent where possible (CREATE IF NOT EXISTS).

### 24.4 Zero-Downtime Migration Patterns

| Change Type              | Safe Pattern                                           |
|--------------------------|--------------------------------------------------------|
| Add Column               | ADD COLUMN with DEFAULT                                |
| Add Index                | CREATE INDEX CONCURRENTLY (PostgreSQL)                 |
| Rename Column            | Add new column → migrate data → drop old column        |
| Drop Column              | Remove application usage → verify → drop in next cycle |
| Add Table                | CREATE TABLE (no impact on existing queries)           |
| Change Data Type         | Add new column → migrate data → switch → drop old      |

---

## 25. Disaster Recovery and Business Continuity

### 25.1 Disaster Recovery Scenarios

| Scenario                        | Impact                           | Recovery Strategy                      |
|---------------------------------|----------------------------------|----------------------------------------|
| Single database server failure  | Service interruption             | Promote standby replica                |
| Storage volume failure          | Potential data loss              | Restore from latest backup + WAL       |
| Data corruption                 | Data integrity compromise        | Point-in-time recovery to pre-corruption state |
| Accidental mass deletion        | Data loss                        | Restore from backup + replay transactions |
| Regional outage                 | Complete service loss            | Failover to DR region                  |
| Ransomware attack               | Data encryption/loss             | Restore from immutable backups         |

### 25.2 Recovery Procedures

**Procedure 1: Database Server Failure (PostgreSQL)**
1. Automated failover via Patroni/etcd (target: < 30 seconds)
2. Verify standby is caught up (replication lag = 0)
3. Update connection pooler configuration
4. Validate application connectivity
5. Notify operations team
6. Investigate root cause
7. Rebuild failed primary as new standby

**Procedure 2: Data Corruption Recovery**
1. Stop application writes to affected tables
2. Identify corruption extent and timestamp
3. Restore latest clean backup to isolated environment
4. Apply WAL archives up to corruption point minus 1 minute
5. Verify data integrity
6. Promote restored database
7. Validate application functionality
8. Conduct post-incident review

**Procedure 3: Accidental Data Deletion**
1. Identify scope and timestamp of deletion
2. Stop application writes (if necessary)
3. Restore latest backup to point-in-time before deletion
4. Apply WAL to restore to state just before deletion
5. Export affected data from restored database
6. Import data into production
7. Verify data integrity
8. Resume normal operations

### 25.3 Backup Testing Schedule

| Test Type                     | Frequency    | Success Criteria                       |
|-------------------------------|--------------|----------------------------------------|
| Backup restoration test       | Weekly       | Full restore within RTO, data integrity verified |
| Point-in-time recovery test   | Monthly      | Recovery to target time, zero data loss |
| DR failover test              | Quarterly    | Successful failover within RTO         |
| Full disaster recovery drill  | Semi-Annual  | Complete system recovery within MTTR   |
| Backup encryption verification| Monthly      | Backup decryption successful           |

---

## 26. Monitoring and Observability

### 26.1 Database Health Metrics

| Metric                          | Collection Interval | Alert Threshold  |
|---------------------------------|---------------------|------------------|
| Connection count                | 10 seconds          | > 80% pool      |
| Query latency (avg/p95/p99)     | 10 seconds          | > 100ms / 500ms |
| Transaction rate                | 10 seconds          | Anomaly detect  |
| Cache hit ratio                 | 60 seconds          | < 95%           |
| Replication lag                 | 10 seconds          | > 5 seconds     |
| Dead tuple count                | 60 seconds          | > 100,000       |
| Disk usage                      | 60 seconds          | > 80%           |
| WAL size                        | 60 seconds          | > 1 GB          |
| Lock wait time                  | 10 seconds          | > 1 second      |
| Failed query rate               | 60 seconds          | > 1%            |
| Backup age                      | 3600 seconds        | > 25 hours      |

### 26.2 Query Logging

| Log Type               | Configuration                                          |
|------------------------|--------------------------------------------------------|
| Slow Query Log         | Queries exceeding 1 second threshold                   |
| Error Log              | All failed queries with error details                  |
| Audit Log              | All DML operations on sensitive tables                 |
| Connection Log         | Connection establishment and release events            |
| Replication Log        | Replication state changes and lag measurements         |

### 26.3 Alerting Rules

| Alert Name                     | Condition                          | Severity  | Notification       |
|--------------------------------|------------------------------------|-----------|--------------------|
| DatabaseConnectionHigh         | Connection pool > 80%              | Warning   | Slack + PagerDuty  |
| DatabaseConnectionCritical     | Connection pool > 95%              | Critical  | PagerDuty          |
| QueryLatencyHigh               | P95 > 500ms for 5 minutes         | Warning   | Slack              |
| QueryLatencyCritical           | P95 > 2000ms for 2 minutes        | Critical  | PagerDuty          |
| ReplicationLagHigh             | Lag > 5 seconds for 2 minutes     | Warning   | Slack              |
| ReplicationLagCritical         | Lag > 30 seconds                   | Critical  | PagerDuty          |
| DiskSpaceLow                   | Usage > 80%                        | Warning   | Slack              |
| DiskSpaceCritical              | Usage > 95%                        | Critical  | PagerDuty          |
| BackupFailed                   | Backup not completed within 25 hours| Critical | PagerDuty          |
| DeadlockDetected               | Any deadlock event                 | Info      | Slack              |

---

## 27. Data Governance and Compliance

### 27.1 Data Classification Governance

| Classification   | Encryption Required | Access Logging | Retention Limit | Sharing Restrictions           |
|------------------|---------------------|----------------|-----------------|--------------------------------|
| RESTRICTED       | At rest + transit   | All access     | Policy-defined  | No external sharing            |
| CONFIDENTIAL     | At rest + transit   | Write access   | Policy-defined  | Need-to-know basis             |
| INTERNAL         | In transit          | Write access   | Policy-defined  | Internal distribution only     |
| PUBLIC           | In transit          | None           | Unlimited       | Unrestricted                   |

### 27.2 Regulatory Compliance

| Regulation        | Requirements                                        | Database Controls                              |
|-------------------|-----------------------------------------------------|-------------------------------------------------|
| GDPR              | Data minimization, right to erasure, consent        | Soft delete, data masking, retention policies   |
| CCPA              | Consumer data access, deletion rights               | Data export, deletion workflows                 |
| SOX               | Audit trail integrity, financial data controls      | Immutable audit logs, access controls           |
| HIPAA             | PHI protection, access logging (if applicable)      | Encryption, access control, audit logging       |
| ISO 27001         | Information security management                     | All controls in this section                    |

### 27.3 Data Residency

| Data Type               | Primary Region        | Backup Region       | Notes                          |
|-------------------------|-----------------------|---------------------|--------------------------------|
| User Data               | Deployment region     | Cross-region        | GDPR: EU user data in EU       |
| Evidence Files          | Same as incident site | DR region           | Minimize latency for playback  |
| Audit Logs              | Deployment region     | Cross-region        | 7-year retention requirement   |
| Detection Events        | Deployment region     | Same region         | High-volume, lower sensitivity |

### 27.4 Right to Erasure (GDPR)

When a user requests data deletion under GDPR Article 17:

1. Verify identity of requesting user
2. Identify all personal data across all entities
3. Anonymize data in audit logs (replace PII with "REDACTED")
4. Soft-delete user account and related sessions
5. Schedule hard deletion after 90-day retention
6. Preserve evidence and incident data (legal obligation) with PII removed
7. Generate deletion confirmation receipt
8. Log deletion action in compliance audit trail

---

## 28. Scalability and Capacity Planning

### 28.1 Capacity Estimates (Per Site)

| Metric                        | 50 Cameras   | 200 Cameras  | 1,000 Cameras | 10,000 Cameras |
|-------------------------------|--------------|--------------|---------------|----------------|
| Detection Events per Day      | 5,000        | 20,000       | 100,000       | 1,000,000      |
| Storage per Day (Events)      | 50 MB        | 200 MB       | 1 GB          | 10 GB          |
| Storage per Day (Evidence)    | 5 GB         | 20 GB        | 100 GB        | 1 TB           |
| Storage per Month (Total)     | 150 GB       | 600 GB       | 3 TB          | 30 TB          |
| Camera Health Records per Day | 72,000       | 288,000      | 1,440,000     | 14,400,000     |
| Audit Log Entries per Day     | 500          | 2,000        | 10,000        | 100,000        |
| Concurrent DB Connections     | 5            | 10           | 30            | 100            |
| IOPS Required (Read)          | 500          | 2,000        | 10,000        | 50,000         |
| IOPS Required (Write)         | 200          | 800          | 4,000         | 20,000         |

### 28.2 Scaling Thresholds

| Scale Level      | Camera Range    | Database Strategy                     | Infrastructure              |
|------------------|-----------------|----------------------------------------|------------------------------|
| Starter          | 1–50            | SQLite, single server                 | Single VM                    |
| Growth           | 51–200          | SQLite, single server with SSD        | Single VM with SSD           |
| Professional     | 201–500         | PostgreSQL, single server             | Dedicated DB server          |
| Enterprise       | 501–2,000       | PostgreSQL, primary + 1 standby       | DB cluster                   |
| Enterprise Plus  | 2,001–5,000     | PostgreSQL, primary + 2 standbys      | DB cluster + read replicas   |
| Hyperscale       | 5,001–10,000+   | PostgreSQL, distributed + Citus       | Multi-region DB deployment   |

### 28.3 Performance Scaling Strategies

| Strategy                        | Trigger Condition                     | Implementation                      |
|---------------------------------|---------------------------------------|--------------------------------------|
| Connection Pool Increase        | Pool utilization > 80%                | Increase PgBouncer pool size         |
| Read Replica Addition           | Read latency > 200ms P95             | Add streaming replica                |
| Table Partitioning              | Table size > 10 GB                   | Declarative range partitioning       |
| Index Optimization              | Query latency > 100ms                | Review and optimize indexes          |
| Materialized View Refresh       | Dashboard load > 2 seconds           | Create/refresh materialized views    |
| Query Optimization              | CPU utilization > 70%                | EXPLAIN ANALYZE, query rewrite       |
| Vertical Scaling                | Hardware limits reached              | Upgrade instance size                |
| Horizontal Scaling              | Write throughput limits reached      | Shard by site_id                     |

---

## 29. Multi-Tenancy Considerations

### 29.1 Multi-Tenancy Model

VigilantAI supports logical multi-tenancy through site-based data isolation. Each tenant is represented by one or more Sites, with data isolation enforced at both the application and database levels.

| Isolation Level       | Implementation                              | Security Guarantee             |
|-----------------------|---------------------------------------------|--------------------------------|
| Application Level     | Repository Pattern with site_id filtering   | Application-layer enforcement  |
| Database Level        | Row-Level Security (PostgreSQL)             | Database-layer enforcement     |
| Physical Level        | Separate database schemas (optional)        | Complete physical isolation    |

### 29.2 Site-Based Data Isolation

All data entities are associated with a Site through direct foreign keys or transitive relationships. Row-Level Security policies enforce that queries automatically filter results to only include data for authorized sites.

**Isolation Boundaries:**
- Users are granted access to specific Sites
- Cameras belong to specific Sites
- Detection Events are scoped to Cameras within Sites
- Incidents are associated with Sites
- Rules are defined per Site
- Audit Logs track which Site was accessed

### 29.3 Cross-Tenancy Operations

| Operation                       | Access Control                              |
|---------------------------------|---------------------------------------------|
| Single-site data access         | User must have site access permission       |
| Cross-site reporting            | Super Admin or Security Admin role required |
| Global system configuration     | Super Admin role only                       |
| Audit log access across sites   | Security Admin or Super Admin role          |
| Evidence export across sites    | Super Admin role with MFA verification      |

---

## 30. Audit Trail and Data Lineage

### 30.1 Audit Trail Architecture

The audit trail provides a complete, immutable record of all significant actions performed within the VigilantAI platform. The audit trail is stored in append-only tables with no UPDATE or DELETE permissions granted to application roles.

**Audit Event Categories:**

| Category               | Events Captured                                        |
|------------------------|--------------------------------------------------------|
| Authentication         | Login success, login failure, logout, MFA verify       |
| Authorization          | Permission denied, role change, access revoked         |
| Data Modification      | Create, update, delete on all business entities        |
| Data Access            | Evidence view, report generation, data export          |
| Configuration Change   | System settings modification, rule changes             |
| Incident Lifecycle     | Create, assign, acknowledge, resolve, close            |
| Evidence Handling      | Capture, verify, export, delete                        |
| System Events          | Backup, migration, failover, restart                   |

### 30.2 Audit Log Immutability

**SQLite:** Audit log tables are created WITHOUT a primary key (UUID only) and application roles have INSERT and SELECT only — no UPDATE or DELETE privileges.

**PostgreSQL:** Audit log tables use REVOKE to remove UPDATE and DELETE permissions from the application role. Additionally, event triggers prevent DDL modifications to audit log tables.

### 30.3 Data Lineage Tracking

Data lineage tracks the origin and transformation history of data as it flows through the system:

| Data Flow                          | Lineage Information                                    |
|------------------------------------|--------------------------------------------------------|
| Camera → Detection Event           | Source camera, timestamp, AI model version             |
| Detection Event → Incident         | Triggering rule, match criteria, severity escalation   |
| Incident → Evidence                | Capture parameters, time window, storage location      |
| Evidence → Evidence Hash           | Hash algorithm, computation timestamp, verification    |
| User Action → Audit Log            | User identity, timestamp, IP address, user agent       |

---

## 31. Testing and Quality Assurance

### 31.1 Database Testing Strategy

| Test Type                  | Scope                          | Frequency        | Tools                  |
|----------------------------|--------------------------------|------------------|------------------------|
| Unit Tests                 | Repository methods, queries    | Every commit     | SQLx compile-time check|
| Integration Tests          | End-to-end data operations     | Every PR         | Test database + fixtures|
| Migration Tests            | Schema migration forward/back  | Every migration  | Separate test DB       |
| Performance Tests          | Query latency, throughput      | Weekly           | pgbench, custom scripts|
| Security Tests             | SQL injection, access control  | Monthly          | OWASP ZAP, manual     |
| Data Integrity Tests       | Constraint validation          | Every commit     | Test data fixtures     |
| Backup/Recovery Tests      | Restore procedures             | Weekly           | Automated restore jobs |

### 31.2 Test Data Management

**Test Data Principles:**
1. Production data is never used in test environments
2. Test data is synthetically generated and anonymized
3. Test databases are reset to known state before each test run
4. Test data includes edge cases and boundary conditions
5. Test data covers all entity types and relationships

### 31.3 Quality Metrics

| Metric                          | Target           | Measurement Frequency  |
|---------------------------------|------------------|------------------------|
| Query test coverage             | > 95%            | Per PR                 |
| Migration success rate          | 100%             | Per migration          |
| Constraint violation rate       | 0 (in production)| Continuous             |
| Slow query count                | < 10 per day     | Daily                  |
| Data integrity check failures   | 0                | Daily                  |

---

## 32. Risk Assessment and Mitigation

### 32.1 Database Risks

| Risk ID  | Risk Description                            | Likelihood | Impact   | Mitigation Strategy                                |
|----------|---------------------------------------------|------------|----------|----------------------------------------------------|
| DR-01    | Database server hardware failure            | Medium     | High     | Streaming replication, automated failover          |
| DR-02    | Data corruption due to software bug         | Low        | Critical | Regular backups, integrity checks, WAL archiving   |
| DR-03    | Ransomware attack encrypting database files | Low        | Critical | Immutable backups, air-gapped backup copies        |
| DR-04    | Accidental mass data deletion               | Medium     | High     | Soft deletion, backup retention, access controls   |
| DR-05    | Performance degradation under load          | Medium     | Medium   | Connection pooling, query optimization, scaling    |
| DR-06    | Schema migration failure                    | Low        | High     | Migration testing, rollback procedures, peer review|
| DR-07    | Encryption key compromise                   | Low        | Critical | Key rotation, HSM storage, access logging          |
| DR-08    | Replication lag causing stale reads         | Medium     | Low      | Monitoring, alerting, synchronous replication for critical data |
| DR-09    | Connection pool exhaustion                  | Medium     | High     | Pool monitoring, query timeouts, connection limits |
| DR-10    | Disk space exhaustion                       | Medium     | High     | Monitoring, automated cleanup, capacity planning   |
| DR-11    | Regulatory non-compliance                   | Low        | Critical | Retention policies, access logging, regular audits |
| DR-12    | Single point of failure in SQLite deployment| High       | High     | Regular backups, migration path to PostgreSQL      |

### 32.2 Risk Monitoring

| Risk Indicator                     | Monitoring Method                    | Escalation Path               |
|------------------------------------|--------------------------------------|--------------------------------|
| Replication lag                    | Continuous monitoring (10s intervals) | Ops → DBA → Engineering Lead  |
| Backup age                         | Continuous monitoring (hourly)        | Ops → DBA → Engineering Lead  |
| Disk space utilization             | Continuous monitoring (minute)        | Ops → DevOps → Engineering Lead|
| Connection pool utilization        | Continuous monitoring (10s intervals) | Ops → DBA                     |
| Query latency degradation          | Continuous monitoring (10s intervals) | DBA → Engineering Lead        |
| Failed backup count                | Daily check                          | Ops → DBA → Engineering Lead  |

---

## 33. Appendices

### Appendix A: Predefined Roles and Permissions Matrix

| Role              | Cameras | Incidents | Evidence | Users | Rules | Reports | Config | Audit |
|-------------------|---------|-----------|----------|-------|-------|---------|--------|-------|
| Super Admin       | CRUD    | CRUD      | CRUD     | CRUD  | CRUD  | CRUD    | CRUD   | R     |
| Security Admin    | R       | CRUD      | CRU      | R     | CRUD  | R       | R      | R     |
| Security Analyst  | R       | RU        | R        | —     | R     | R       | —      | —     |
| Operator          | R       | CRU       | CR       | —     | R     | R       | —      | —     |
| Viewer            | R       | R         | R        | —     | R     | R       | —      | —     |
| API Integration   | R       | R         | R        | —     | R     | R       | —      | —     |

**Legend:** C = Create, R = Read, U = Update, D = Delete, — = No Access

### Appendix B: JSONB Schema Examples

**Detection Event detected_objects Schema:**
```json
{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "class": { "type": "string" },
      "confidence": { "type": "number", "minimum": 0, "maximum": 1 },
      "track_id": { "type": "integer" },
      "attributes": { "type": "object" }
    }
  }
}
```

**Rule conditions Schema:**
```json
{
  "type": "object",
  "properties": {
    "zone_id": { "type": "string" },
    "object_classes": { "type": "array", "items": { "type": "string" } },
    "min_confidence": { "type": "number" },
    "min_duration_seconds": { "type": "integer" },
    "time_range": {
      "type": "object",
      "properties": {
        "start": { "type": "string", "format": "time" },
        "end": { "type": "string", "format": "time" }
      }
    }
  }
}
```

### Appendix C: Migration Checklist

- [ ] Migration SQL is syntactically valid for both SQLite and PostgreSQL
- [ ] Migration has been tested on both database engines
- [ ] Migration is backward compatible with current application version
- [ ] Rollback migration has been written and tested
- [ ] Migration does not lock tables for extended periods
- [ ] Data migration (if applicable) handles edge cases
- [ ] Performance impact has been assessed
- [ ] Migration has been peer reviewed
- [ ] Migration file is properly versioned and named
- [ ] Documentation has been updated if schema changes affect API

### Appendix D: Database Connection Configuration

**SQLite Configuration:**
```
DATABASE_URL=sqlite:vigilant.db?mode=rwc
SQLITE_WAL_MODE=true
SQLITE_BUSY_TIMEOUT=5000
SQLITE_FOREIGN_KEYS=on
```

**PostgreSQL Configuration:**
```
DATABASE_URL=postgres://vigilant:password@localhost:5432/vigilantai
DATABASE_POOL_SIZE=20
DATABASE_MAX_CONNECTIONS=200
DATABASE_CONNECT_TIMEOUT=5
DATABASE_IDLE_TIMEOUT=300
DATABASE_SQLX_TLS=require
```

### Appendix E: Glossary of Database-Specific Terms

| Term                      | Definition                                              |
|---------------------------|---------------------------------------------------------|
| BRIN Index                | Block Range Index — space-efficient index for large ordered tables |
| GIN Index                 | Generalized Inverted Index — supports composite types, arrays, JSONB |
| GiST Index                | Generalized Search Tree — supports geometric and全文 data types |
| MVCC                      | Multi-Version Concurrency Control — non-blocking read mechanism |
| TOAST                    | The Oversized-Attribute Storage Technique — PostgreSQL large value storage |
| WAL                       | Write-Ahead Logging — crash recovery mechanism           |
| VACUUM                    | PostgreSQL maintenance command to reclaim storage        |
| ANALYZE                   | PostgreSQL command to update query planner statistics    |
| PgBouncer                 | Lightweight connection pooler for PostgreSQL             |
| SQLCipher                 | SQLite extension providing transparent AES-256 encryption |
| SQLx                      | Rust SQL toolkit with compile-time query verification    |
| Partition Pruning         | Query optimization that excludes irrelevant partitions   |
| Streaming Replication     | Real-time PostgreSQL data synchronization                |
| Logical Replication       | Selective PostgreSQL replication by table or filter      |
| FTS5                      | SQLite full-text search extension                       |
| JSON1                     | SQLite JSON parsing extension                           |

---

*End of Document*
