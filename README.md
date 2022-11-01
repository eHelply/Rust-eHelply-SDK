# Rust API client for ehelply

eHelply SDK for SuperStack Services

For more information, please visit [https://superstack.ehelply.com/support](https://superstack.ehelply.com/support)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.1.117
- Package version: 1.1.117
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `ehelply` and add the following to `Cargo.toml` under `[dependencies]`:

```
ehelply = { path = "./ehelply" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.prod.ehelply.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AppointmentsApi* | [**add_entity_to_appointment**](docs/AppointmentsApi.md#add_entity_to_appointment) | **POST** /appointments/appointments/{appointment_uuid}/entities/{entity_uuid} | Addentitytoappointment
*AppointmentsApi* | [**create_appointment**](docs/AppointmentsApi.md#create_appointment) | **POST** /appointments/appointments | Createappointment
*AppointmentsApi* | [**delete_appointment**](docs/AppointmentsApi.md#delete_appointment) | **DELETE** /appointments/appointments/{appointment_uuid} | Deleteappointment
*AppointmentsApi* | [**detach_entity_from_appointment**](docs/AppointmentsApi.md#detach_entity_from_appointment) | **DELETE** /appointments/appointments/{appointment_uuid}/entities/{entity_uuid} | Removeentityfromappointment
*AppointmentsApi* | [**get_appointment**](docs/AppointmentsApi.md#get_appointment) | **GET** /appointments/appointments/{appointment_uuid} | Getappointment
*AppointmentsApi* | [**search_appointment**](docs/AppointmentsApi.md#search_appointment) | **GET** /appointments/appointments | Searchappointments
*AppointmentsApi* | [**search_appointment_entities**](docs/AppointmentsApi.md#search_appointment_entities) | **GET** /appointments/appointments/{appointment_uuid}/entities | Searchappointmententities
*AppointmentsApi* | [**search_entity_appointments**](docs/AppointmentsApi.md#search_entity_appointments) | **GET** /appointments/appointments/entities/{entity_uuid}/appointments | Getentityappointments
*AppointmentsApi* | [**update_appointment**](docs/AppointmentsApi.md#update_appointment) | **PUT** /appointments/appointments/{appointment_uuid} | Updateappointment
*BillingApi* | [**create_billing_account**](docs/BillingApi.md#create_billing_account) | **POST** /sam/billing/projects/{project_uuid}/accounts | Createbillingaccount
*BillingApi* | [**get_client_secret**](docs/BillingApi.md#get_client_secret) | **GET** /sam/billing/projects/{project_uuid}/secrets | Getclientsecret
*BillingApi* | [**has_payment**](docs/BillingApi.md#has_payment) | **GET** /sam/billing/projects/{project_uuid}/payment-methods-exist | Haspayment
*BillingApi* | [**list_payment_methods**](docs/BillingApi.md#list_payment_methods) | **GET** /sam/billing/projects/{project_uuid}/payment-methods | Listpaymentmethods
*BillingApi* | [**process_payment**](docs/BillingApi.md#process_payment) | **POST** /sam/billing/projects/{project_uuid}/payments | Processpayment
*BillingApi* | [**reconcile_payment_method**](docs/BillingApi.md#reconcile_payment_method) | **GET** /sam/billing/projects/{project_uuid}/payment-methods-reconciliation | Reconcilepaymentmethod
*BillingApi* | [**remove_payment_method**](docs/BillingApi.md#remove_payment_method) | **DELETE** /sam/billing/projects/{project_uuid}/payment-methods | Removepaymentmethod
*CatalogsApi* | [**attach_product_to_catalog**](docs/CatalogsApi.md#attach_product_to_catalog) | **POST** /products/catalogs/{catalog_uuid}/products/{product_uuid} | Addproducttocatalog
*CatalogsApi* | [**create_catalog**](docs/CatalogsApi.md#create_catalog) | **POST** /products/catalogs | Createcatalog
*CatalogsApi* | [**delete_catalog**](docs/CatalogsApi.md#delete_catalog) | **DELETE** /products/catalogs/{catalog_uuid} | Deletecatalog
*CatalogsApi* | [**detach_product_from_catalog**](docs/CatalogsApi.md#detach_product_from_catalog) | **DELETE** /products/catalogs/{catalog_uuid}/products/{product_uuid} | Removeproductfromcatalog
*CatalogsApi* | [**get_catalog**](docs/CatalogsApi.md#get_catalog) | **GET** /products/catalogs/{catalog_uuid} | Getcatalog
*CatalogsApi* | [**search_catalog_products**](docs/CatalogsApi.md#search_catalog_products) | **GET** /products/catalogs/{catalog_uuid}/products | Searchcatalogproducts
*CatalogsApi* | [**search_catalogs**](docs/CatalogsApi.md#search_catalogs) | **GET** /products/catalogs | Searchcatalogs
*CatalogsApi* | [**update_catalog**](docs/CatalogsApi.md#update_catalog) | **PUT** /products/catalogs/{catalog_uuid} | Updatecatalog
*CategoryApi* | [**create_category_places_categories_post**](docs/CategoryApi.md#create_category_places_categories_post) | **POST** /places/categories | Create Category
*CategoryApi* | [**delete_category_places_categories_category_uuid_delete**](docs/CategoryApi.md#delete_category_places_categories_category_uuid_delete) | **DELETE** /places/categories/{category_uuid} | Delete Category
*CategoryApi* | [**get_category_places_categories_category_uuid_get**](docs/CategoryApi.md#get_category_places_categories_category_uuid_get) | **GET** /places/categories/{category_uuid} | Get Category
*CategoryApi* | [**search_categories_places_categories_get**](docs/CategoryApi.md#search_categories_places_categories_get) | **GET** /places/categories | Search Categories
*CategoryApi* | [**update_category_places_categories_category_uuid_put**](docs/CategoryApi.md#update_category_places_categories_category_uuid_put) | **PUT** /places/categories/{category_uuid} | Update Category
*CompaniesApi* | [**create_company_places_companies_post**](docs/CompaniesApi.md#create_company_places_companies_post) | **POST** /places/companies | Create Company
*CompaniesApi* | [**delete_place_places_companies_company_uuid_delete**](docs/CompaniesApi.md#delete_place_places_companies_company_uuid_delete) | **DELETE** /places/companies/{company_uuid} | Delete Place
*CompaniesApi* | [**get_company_places_companies_company_uuid_get**](docs/CompaniesApi.md#get_company_places_companies_company_uuid_get) | **GET** /places/companies/{company_uuid} | Get Company
*CompaniesApi* | [**search_companies_places_companies_get**](docs/CompaniesApi.md#search_companies_places_companies_get) | **GET** /places/companies | Search Companies
*CompaniesApi* | [**update_company_places_companies_company_uuid_put**](docs/CompaniesApi.md#update_company_places_companies_company_uuid_put) | **PUT** /places/companies/{company_uuid} | Update Company
*ContentApi* | [**create_file**](docs/ContentApi.md#create_file) | **POST** /files/files | Createfile
*ContentApi* | [**delete_file**](docs/ContentApi.md#delete_file) | **DELETE** /files/files/{file_uuid} | Deletefile
*ContentApi* | [**get_file**](docs/ContentApi.md#get_file) | **GET** /files/files/{file_uuid} | Getfile
*ContentApi* | [**update_file**](docs/ContentApi.md#update_file) | **PUT** /files/files/{file_uuid} | Updatefile
*FactsApi* | [**delete_fact**](docs/FactsApi.md#delete_fact) | **POST** /sam/facts/facts/{fact_name} | Deletefact
*FactsApi* | [**delete_fact_0**](docs/FactsApi.md#delete_fact_0) | **POST** /sam/facts/facts/{fact_name} | Deletefact
*FactsApi* | [**get_fact**](docs/FactsApi.md#get_fact) | **GET** /sam/facts/facts/{fact_name} | Getfact
*FactsApi* | [**get_fact_0**](docs/FactsApi.md#get_fact_0) | **GET** /sam/facts/facts/{fact_name} | Getfact
*FactsApi* | [**get_facts**](docs/FactsApi.md#get_facts) | **GET** /sam/facts/facts | Getfacts
*FactsApi* | [**get_facts_0**](docs/FactsApi.md#get_facts_0) | **GET** /sam/facts/facts | Getfacts
*FactsApi* | [**save_fact**](docs/FactsApi.md#save_fact) | **POST** /sam/facts/facts | Savefact
*FactsApi* | [**save_fact_0**](docs/FactsApi.md#save_fact_0) | **POST** /sam/facts/facts | Savefact
*FieldsApi* | [**create_field**](docs/FieldsApi.md#create_field) | **POST** /fields/fields | Createfield
*FieldsApi* | [**delete_field**](docs/FieldsApi.md#delete_field) | **DELETE** /fields/fields/{field_uuid} | Deletefield
*FieldsApi* | [**get_field**](docs/FieldsApi.md#get_field) | **GET** /fields/fields/{field_uuid} | Getfield
*FieldsApi* | [**update_field**](docs/FieldsApi.md#update_field) | **PUT** /fields/fields/{field_uuid} | Updatefield
*LoggingApi* | [**get_subject_logs**](docs/LoggingApi.md#get_subject_logs) | **GET** /sam/logging/logs/services/{service}/subjects/{subject} | Getsubjectlogs
*MetaApi* | [**create_meta**](docs/MetaApi.md#create_meta) | **POST** /meta/meta/service/{service}/type/{type_name}/entity/{entity_uuid} | Createmeta
*MetaApi* | [**create_slug**](docs/MetaApi.md#create_slug) | **POST** /meta/slug | Createslug
*MetaApi* | [**delete_meta**](docs/MetaApi.md#delete_meta) | **DELETE** /meta/meta/{meta_uuid} | Deletemeta
*MetaApi* | [**delete_meta_from_parts**](docs/MetaApi.md#delete_meta_from_parts) | **DELETE** /meta/meta/service/{service}/type/{type_name}/entity/{entity_uuid} | Deletemetafromparts
*MetaApi* | [**get_meta**](docs/MetaApi.md#get_meta) | **GET** /meta/meta/{meta_uuid} | Getmeta
*MetaApi* | [**get_meta_from_parts**](docs/MetaApi.md#get_meta_from_parts) | **GET** /meta/meta/service/{service}/type/{type_name}/entity/{entity_uuid} | Getmetafromparts
*MetaApi* | [**touch_meta**](docs/MetaApi.md#touch_meta) | **POST** /meta/meta/{meta_uuid}/touch | Touchmeta
*MetaApi* | [**update_meta**](docs/MetaApi.md#update_meta) | **PUT** /meta/meta/{meta_uuid} | Updatemeta
*MetaApi* | [**update_meta_from_parts**](docs/MetaApi.md#update_meta_from_parts) | **PUT** /meta/meta/service/{service}/type/{type_name}/entity/{entity_uuid} | Updatemetafromparts
*MonitorApi* | [**acknowledge_alarm**](docs/MonitorApi.md#acknowledge_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/acknowledge | Acknowledgealarm
*MonitorApi* | [**assign_alarm**](docs/MonitorApi.md#assign_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/assign | Assignalarm
*MonitorApi* | [**attach_alarm_note**](docs/MonitorApi.md#attach_alarm_note) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/note | Attachalarmnote
*MonitorApi* | [**attach_alarm_ticket**](docs/MonitorApi.md#attach_alarm_ticket) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/ticket | Attachalarmticket
*MonitorApi* | [**clear_alarm**](docs/MonitorApi.md#clear_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/clear | Clearalarm
*MonitorApi* | [**delete_service_super_stack_meta**](docs/MonitorApi.md#delete_service_super_stack_meta) | **DELETE** /sam/monitor/services/{service}/superstack | Deleteservicesuperstackmeta
*MonitorApi* | [**get_service**](docs/MonitorApi.md#get_service) | **GET** /sam/monitor/services/{service} | Getservice
*MonitorApi* | [**get_service_alarm**](docs/MonitorApi.md#get_service_alarm) | **GET** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid} | Getservicealarm
*MonitorApi* | [**get_service_alarms**](docs/MonitorApi.md#get_service_alarms) | **GET** /sam/monitor/services/{service}/stages/{stage}/alarms | Getservicealarms
*MonitorApi* | [**get_service_heartbeat**](docs/MonitorApi.md#get_service_heartbeat) | **GET** /sam/monitor/services/{service}/stages/{stage}/heartbeats | Getserviceheartbeat
*MonitorApi* | [**get_service_kpis**](docs/MonitorApi.md#get_service_kpis) | **GET** /sam/monitor/services/{service}/kpis | Getservicekpis
*MonitorApi* | [**get_service_spec**](docs/MonitorApi.md#get_service_spec) | **GET** /sam/monitor/services/{service}/specs/{spec} | Getservicespec
*MonitorApi* | [**get_service_specs**](docs/MonitorApi.md#get_service_specs) | **GET** /sam/monitor/services/{service}/specs | Getservicespecs
*MonitorApi* | [**get_service_vitals**](docs/MonitorApi.md#get_service_vitals) | **GET** /sam/monitor/services/{service}/stages/{stage}/vitals | Getservicevitals
*MonitorApi* | [**get_services**](docs/MonitorApi.md#get_services) | **GET** /sam/monitor/services | Getservices
*MonitorApi* | [**get_services_with_specs**](docs/MonitorApi.md#get_services_with_specs) | **GET** /sam/monitor/specs/services | Getserviceswithspecs
*MonitorApi* | [**get_supertack_services**](docs/MonitorApi.md#get_supertack_services) | **GET** /sam/monitor/superstack-services | Getsupertackservices
*MonitorApi* | [**hide_service**](docs/MonitorApi.md#hide_service) | **POST** /sam/monitor/services/{service}/stages/{stage}/hide | Hideservice
*MonitorApi* | [**ignore_alarm**](docs/MonitorApi.md#ignore_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/ignore | Ignorealarm
*MonitorApi* | [**register_service**](docs/MonitorApi.md#register_service) | **POST** /sam/monitor/services | Registerservice
*MonitorApi* | [**save_service_super_stack_meta**](docs/MonitorApi.md#save_service_super_stack_meta) | **POST** /sam/monitor/services/{service}/superstack | Saveservicesuperstackmeta
*MonitorApi* | [**search_alarms**](docs/MonitorApi.md#search_alarms) | **GET** /sam/monitor/services/{service}/alarms | Searchalarms
*MonitorApi* | [**show_service**](docs/MonitorApi.md#show_service) | **POST** /sam/monitor/services/{service}/stages/{stage}/show | Showservice
*MonitorApi* | [**terminate_alarm**](docs/MonitorApi.md#terminate_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms/{alarm_uuid}/terminate | Terminatealarm
*MonitorApi* | [**trigger_alarm**](docs/MonitorApi.md#trigger_alarm) | **POST** /sam/monitor/services/{service}/stages/{stage}/alarms | Triggeralarm
*NotesApi* | [**create_note**](docs/NotesApi.md#create_note) | **POST** /notes/notes | Createnote
*NotesApi* | [**delete_note**](docs/NotesApi.md#delete_note) | **DELETE** /notes/notes/{note_id} | Deletenote
*NotesApi* | [**get_note**](docs/NotesApi.md#get_note) | **GET** /notes/notes/{note_id} | Getnote
*NotesApi* | [**update_note**](docs/NotesApi.md#update_note) | **PUT** /notes/notes/{note_id} | Updatenote
*PlacesApi* | [**advanced_search_places**](docs/PlacesApi.md#advanced_search_places) | **GET** /places/search/places/string | Advancedsearchplaces
*PlacesApi* | [**create_place_places_places_post**](docs/PlacesApi.md#create_place_places_places_post) | **POST** /places/places | Create Place
*PlacesApi* | [**delete_place**](docs/PlacesApi.md#delete_place) | **DELETE** /places/places/{place_uuid} | Deleteplace
*PlacesApi* | [**forward_geocoding_places_geocoding_forward_get**](docs/PlacesApi.md#forward_geocoding_places_geocoding_forward_get) | **GET** /places/geocoding/forward | Forward Geocoding
*PlacesApi* | [**get_place**](docs/PlacesApi.md#get_place) | **GET** /places/places/{place_uuid} | Getplace
*PlacesApi* | [**reverse_geocoding_places_geocoding_reverse_get**](docs/PlacesApi.md#reverse_geocoding_places_geocoding_reverse_get) | **GET** /places/geocoding/reverse | Reverse Geocoding
*PlacesApi* | [**search_places**](docs/PlacesApi.md#search_places) | **GET** /places/places | Searchplaces
*PlacesApi* | [**update_place**](docs/PlacesApi.md#update_place) | **PUT** /places/places/{place_uuid} | Updateplace
*ProductsApi* | [**create_product**](docs/ProductsApi.md#create_product) | **POST** /products/products | Createproduct
*ProductsApi* | [**delete_product**](docs/ProductsApi.md#delete_product) | **DELETE** /products/products/{product_uuid} | Deleteproduct
*ProductsApi* | [**get_product**](docs/ProductsApi.md#get_product) | **GET** /products/products/{product_uuid} | Getproduct
*ProductsApi* | [**search_product_catalog**](docs/ProductsApi.md#search_product_catalog) | **GET** /products/products/{product_uuid}/catalogs | Searchproductcatalog
*ProductsApi* | [**search_products**](docs/ProductsApi.md#search_products) | **GET** /products/products | Searchproducts
*ProductsApi* | [**update_product**](docs/ProductsApi.md#update_product) | **PUT** /products/products/{product_uuid} | Updateproduct
*ProjectsApi* | [**add_member_to_project**](docs/ProjectsApi.md#add_member_to_project) | **POST** /sam/projects/projects/{project_uuid}/members/{entity_uuid} | Addmembertoproject
*ProjectsApi* | [**archive_project**](docs/ProjectsApi.md#archive_project) | **DELETE** /sam/projects/projects/{project_uuid} | Archiveproject
*ProjectsApi* | [**create_project**](docs/ProjectsApi.md#create_project) | **POST** /sam/projects/projects | Createproject
*ProjectsApi* | [**create_project_credential**](docs/ProjectsApi.md#create_project_credential) | **POST** /sam/projects/projects/{project_uuid}/credentials | Createprojectcredential
*ProjectsApi* | [**create_project_credit**](docs/ProjectsApi.md#create_project_credit) | **POST** /sam/projects/projects/{project_uuid}/credits | Createprojectcredit
*ProjectsApi* | [**create_project_invoice**](docs/ProjectsApi.md#create_project_invoice) | **POST** /sam/projects/projects/{project_uuid}/invoices | Createprojectinvoice
*ProjectsApi* | [**create_project_key**](docs/ProjectsApi.md#create_project_key) | **POST** /sam/projects/projects/{project_uuid}/keys | Createprojectkey
*ProjectsApi* | [**create_usage_type**](docs/ProjectsApi.md#create_usage_type) | **POST** /sam/projects/usage/types | Createusagetype
*ProjectsApi* | [**delete_project_credential**](docs/ProjectsApi.md#delete_project_credential) | **DELETE** /sam/projects/projects/{project_uuid}/credentials/{service_name} | Deleteprojectcredential
*ProjectsApi* | [**delete_project_key**](docs/ProjectsApi.md#delete_project_key) | **DELETE** /sam/projects/projects/{project_uuid}/keys | Deleteprojectkey
*ProjectsApi* | [**delete_usage_type**](docs/ProjectsApi.md#delete_usage_type) | **DELETE** /sam/projects/usage/types/{usage_type_key} | Deleteusagetype
*ProjectsApi* | [**get_all_project_credentials**](docs/ProjectsApi.md#get_all_project_credentials) | **GET** /sam/projects/projects/{project_uuid}/credentials | Getallprojectcredentials
*ProjectsApi* | [**get_all_project_credits**](docs/ProjectsApi.md#get_all_project_credits) | **GET** /sam/projects/projects/{project_uuid}/credits | Getallprojectcredits
*ProjectsApi* | [**get_all_project_usage**](docs/ProjectsApi.md#get_all_project_usage) | **GET** /sam/projects/projects/{project_uuid}/usage | Getallprojectusage
*ProjectsApi* | [**get_member_projects**](docs/ProjectsApi.md#get_member_projects) | **GET** /sam/projects/members/{entity_uuid}/projects | Getmemberprojects
*ProjectsApi* | [**get_project**](docs/ProjectsApi.md#get_project) | **GET** /sam/projects/projects/{project_uuid} | Getproject
*ProjectsApi* | [**get_project_credit_transactions**](docs/ProjectsApi.md#get_project_credit_transactions) | **GET** /sam/projects/projects/{project_uuid}/credits/{credit_uuid}/transactions | Getprojectcredittransactions
*ProjectsApi* | [**get_project_invoice**](docs/ProjectsApi.md#get_project_invoice) | **GET** /sam/projects/projects/{project_uuid}/invoices | Getprojectinvoice
*ProjectsApi* | [**get_project_invoice_history**](docs/ProjectsApi.md#get_project_invoice_history) | **GET** /sam/projects/projects/{project_uuid}/invoices/history | Getprojectinvoicehistory
*ProjectsApi* | [**get_project_keys**](docs/ProjectsApi.md#get_project_keys) | **GET** /sam/projects/projects/{project_uuid}/keys | Getprojectkeys
*ProjectsApi* | [**get_project_members**](docs/ProjectsApi.md#get_project_members) | **GET** /sam/projects/projects/{project_uuid}/members | Getprojectmembers
*ProjectsApi* | [**get_specific_project_credential**](docs/ProjectsApi.md#get_specific_project_credential) | **GET** /sam/projects/projects/{project_uuid}/credentials/{service_name} | Getspecificprojectcredential
*ProjectsApi* | [**get_specific_project_usage**](docs/ProjectsApi.md#get_specific_project_usage) | **GET** /sam/projects/projects/{project_uuid}/usage/{usage_type_key} | Getspecificprojectusage
*ProjectsApi* | [**get_usage_type**](docs/ProjectsApi.md#get_usage_type) | **GET** /sam/projects/usage/types/{usage_type_key} | Getusagetype
*ProjectsApi* | [**remove_member_from_project**](docs/ProjectsApi.md#remove_member_from_project) | **DELETE** /sam/projects/projects/{project_uuid}/members/{entity_uuid} | Removememberfromproject
*ProjectsApi* | [**revoke_project_credit**](docs/ProjectsApi.md#revoke_project_credit) | **DELETE** /sam/projects/projects/{project_uuid}/credits/{credit_uuid} | Revokeprojectcredit
*ProjectsApi* | [**search_projects**](docs/ProjectsApi.md#search_projects) | **GET** /sam/projects/projects | Searchprojects
*ProjectsApi* | [**search_usage_type**](docs/ProjectsApi.md#search_usage_type) | **GET** /sam/projects/usage/types | Searchusagetype
*ProjectsApi* | [**update_project**](docs/ProjectsApi.md#update_project) | **PUT** /sam/projects/projects/{project_uuid} | Updateproject
*ProjectsApi* | [**update_project_credential**](docs/ProjectsApi.md#update_project_credential) | **PUT** /sam/projects/projects/{project_uuid}/credentials/{service_name} | Updateprojectcredential
*ProjectsApi* | [**update_usage_type**](docs/ProjectsApi.md#update_usage_type) | **PUT** /sam/projects/usage/types/{usage_type_key} | Updateusagetype
*ReviewsApi* | [**create_review**](docs/ReviewsApi.md#create_review) | **POST** /products/reviews/types/{entity_type}/entities/{entity_uuid} | Create
*ReviewsApi* | [**delete_review**](docs/ReviewsApi.md#delete_review) | **DELETE** /products/reviews/types/{entity_type}/entities/{entity_uuid}/reviews/{review_uuid} | Deletereview
*ReviewsApi* | [**get_review**](docs/ReviewsApi.md#get_review) | **GET** /products/reviews/types/{entity_type}/entities/{entity_uuid}/reviews/{review_uuid} | Getreview
*ReviewsApi* | [**search_reviews**](docs/ReviewsApi.md#search_reviews) | **GET** /products/reviews/types/{entity_type}/entities/{entity_uuid} | Searchreview
*ReviewsApi* | [**update_review**](docs/ReviewsApi.md#update_review) | **PUT** /products/reviews/types/{entity_type}/entities/{entity_uuid}/reviews/{review_uuid} | Updatereview
*SecurityApi* | [**create_encryption_key**](docs/SecurityApi.md#create_encryption_key) | **POST** /sam/security/encryption/categories/{category}/keys | Createencryptionkey
*SecurityApi* | [**create_key**](docs/SecurityApi.md#create_key) | **POST** /sam/security/keys | Createkey
*SecurityApi* | [**delete_key**](docs/SecurityApi.md#delete_key) | **DELETE** /sam/security/keys/{key_uuid} | Deletekey
*SecurityApi* | [**generate_token**](docs/SecurityApi.md#generate_token) | **POST** /sam/security/tokens | Generatetoken
*SecurityApi* | [**get_encryption_key**](docs/SecurityApi.md#get_encryption_key) | **GET** /sam/security/encryption/categories/{category}/keys | Getencryptionkey
*SecurityApi* | [**get_key**](docs/SecurityApi.md#get_key) | **GET** /sam/security/keys/{key_uuid} | Getkey
*SecurityApi* | [**search_keys**](docs/SecurityApi.md#search_keys) | **GET** /sam/security/keys | Searchkeys
*SecurityApi* | [**verify_key**](docs/SecurityApi.md#verify_key) | **POST** /sam/security/keys/verify | Verifykey
*StaffApi* | [**create_staff**](docs/StaffApi.md#create_staff) | **POST** /places/staff | Createstaff
*StaffApi* | [**delete_staff**](docs/StaffApi.md#delete_staff) | **DELETE** /places/staff/{staff_uuid} | Deletestaff
*StaffApi* | [**get_staff**](docs/StaffApi.md#get_staff) | **GET** /places/staff/{staff_uuid} | Getstaff
*StaffApi* | [**search_staff**](docs/StaffApi.md#search_staff) | **GET** /places/staff | Searchstaff
*StaffApi* | [**update_staff**](docs/StaffApi.md#update_staff) | **PUT** /places/staff/{staff_uuid} | Updatestaff
*SupportApi* | [**create_contact**](docs/SupportApi.md#create_contact) | **POST** /sam/support/contact | Createcontact
*SupportApi* | [**create_ticket**](docs/SupportApi.md#create_ticket) | **POST** /sam/support/projects/{project_uuid}/members/{member_uuid}/tickets | Createticket
*SupportApi* | [**list_tickets**](docs/SupportApi.md#list_tickets) | **GET** /sam/support/projects/{project_uuid}/members/{member_uuid}/tickets | Listtickets
*SupportApi* | [**update_ticket**](docs/SupportApi.md#update_ticket) | **PUT** /sam/support/projects/{project_uuid}/members/{member_uuid}/tickets/{ticket_id} | Updateticket
*SupportApi* | [**view_ticket**](docs/SupportApi.md#view_ticket) | **GET** /sam/support/projects/{project_uuid}/members/{member_uuid}/tickets/{ticket_id} | Viewticket
*TagApi* | [**delete_tag**](docs/TagApi.md#delete_tag) | **DELETE** /places/tags/{tag_uuid} | Deletetag
*TagsApi* | [**create_tag**](docs/TagsApi.md#create_tag) | **POST** /places/tags | Createtag
*TagsApi* | [**get_tag**](docs/TagsApi.md#get_tag) | **GET** /places/tags/{tag_uuid} | Gettag
*TagsApi* | [**search_tag**](docs/TagsApi.md#search_tag) | **GET** /places/tags | Searchtag
*TagsApi* | [**update_tag**](docs/TagsApi.md#update_tag) | **PUT** /places/tags/{tag_uuid} | Updatetag
*UsersApi* | [**confirm_signup**](docs/UsersApi.md#confirm_signup) | **POST** /sam/users/auth/signup/confirm | Confirmsignup
*UsersApi* | [**create_participant**](docs/UsersApi.md#create_participant) | **POST** /sam/users/participants | Createparticipant
*UsersApi* | [**create_user**](docs/UsersApi.md#create_user) | **POST** /sam/users | Createuser
*UsersApi* | [**delete_participant**](docs/UsersApi.md#delete_participant) | **DELETE** /sam/users/participants/{participant_id} | Deleteparticipant
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /sam/users/{user_id} | Deleteuser
*UsersApi* | [**get_participant**](docs/UsersApi.md#get_participant) | **GET** /sam/users/participants/{participant_id} | Getparticipant
*UsersApi* | [**get_user**](docs/UsersApi.md#get_user) | **GET** /sam/users/{user_id} | Getuser
*UsersApi* | [**login**](docs/UsersApi.md#login) | **POST** /sam/users/auth/login | Login
*UsersApi* | [**refresh_token**](docs/UsersApi.md#refresh_token) | **POST** /sam/users/auth/{app_client}/refresh-token | Refreshtoken
*UsersApi* | [**reset_password**](docs/UsersApi.md#reset_password) | **POST** /sam/users/auth/password/reset | Resetpassword
*UsersApi* | [**reset_password_confirmation**](docs/UsersApi.md#reset_password_confirmation) | **POST** /sam/users/auth/password/reset/confirm | Resetpasswordconfirmation
*UsersApi* | [**search_participants**](docs/UsersApi.md#search_participants) | **GET** /sam/users/participants | Searchparticipants
*UsersApi* | [**signup**](docs/UsersApi.md#signup) | **POST** /sam/users/auth/signup | Signup
*UsersApi* | [**update_participant**](docs/UsersApi.md#update_participant) | **PUT** /sam/users/participants/{participant_id} | Updateparticipant
*UsersApi* | [**update_user**](docs/UsersApi.md#update_user) | **PUT** /sam/users/{user_id} | Updateuser
*UsersApi* | [**user_validations**](docs/UsersApi.md#user_validations) | **POST** /sam/users/validations/{field} | Uservalidations


## Documentation For Models

 - [AddressBase](docs/AddressBase.md)
 - [AlarmAcknowledge](docs/AlarmAcknowledge.md)
 - [AlarmAssign](docs/AlarmAssign.md)
 - [AlarmCreate](docs/AlarmCreate.md)
 - [AlarmIgnore](docs/AlarmIgnore.md)
 - [AlarmNote](docs/AlarmNote.md)
 - [AlarmResponse](docs/AlarmResponse.md)
 - [AlarmTerminate](docs/AlarmTerminate.md)
 - [AlarmTicket](docs/AlarmTicket.md)
 - [AppointmentBase](docs/AppointmentBase.md)
 - [AppointmentResponse](docs/AppointmentResponse.md)
 - [AttachPaymentToProject](docs/AttachPaymentToProject.md)
 - [Basic](docs/Basic.md)
 - [BasicMeta](docs/BasicMeta.md)
 - [BasicMetaCreate](docs/BasicMetaCreate.md)
 - [CatalogBase](docs/CatalogBase.md)
 - [CatalogReturn](docs/CatalogReturn.md)
 - [CategoryBase](docs/CategoryBase.md)
 - [CategoryDb](docs/CategoryDb.md)
 - [Company](docs/Company.md)
 - [CompanyBase](docs/CompanyBase.md)
 - [CompanyResponse](docs/CompanyResponse.md)
 - [Contact](docs/Contact.md)
 - [ContactBase](docs/ContactBase.md)
 - [ContactMethod](docs/ContactMethod.md)
 - [ContactResponse](docs/ContactResponse.md)
 - [CreateField200Response](docs/CreateField200Response.md)
 - [CreateFile200Response](docs/CreateFile200Response.md)
 - [CreateKeyResponse](docs/CreateKeyResponse.md)
 - [CreateMeta200Response](docs/CreateMeta200Response.md)
 - [CreateNote200Response](docs/CreateNote200Response.md)
 - [CreateProjectCredential](docs/CreateProjectCredential.md)
 - [CreateProjectCredit](docs/CreateProjectCredit.md)
 - [CreateProjectInvoice](docs/CreateProjectInvoice.md)
 - [CreateReview](docs/CreateReview.md)
 - [CreateSlug200Response](docs/CreateSlug200Response.md)
 - [CreateTicket](docs/CreateTicket.md)
 - [Credential](docs/Credential.md)
 - [CustomList](docs/CustomList.md)
 - [DatesMeta](docs/DatesMeta.md)
 - [DeleteFact200Response](docs/DeleteFact200Response.md)
 - [DeleteField200Response](docs/DeleteField200Response.md)
 - [DeleteFile200Response](docs/DeleteFile200Response.md)
 - [DeleteMeta200Response](docs/DeleteMeta200Response.md)
 - [DeleteNote200Response](docs/DeleteNote200Response.md)
 - [Detailed](docs/Detailed.md)
 - [DetailedMeta](docs/DetailedMeta.md)
 - [DetailedMetaCreate](docs/DetailedMetaCreate.md)
 - [Discount](docs/Discount.md)
 - [Email](docs/Email.md)
 - [Fact](docs/Fact.md)
 - [FactCreate](docs/FactCreate.md)
 - [Field](docs/Field.md)
 - [FieldDynamo](docs/FieldDynamo.md)
 - [GetAppointment403Response](docs/GetAppointment403Response.md)
 - [GetInvoiceResponse](docs/GetInvoiceResponse.md)
 - [GetProjectCredential](docs/GetProjectCredential.md)
 - [GetProjectInvoiceHistory](docs/GetProjectInvoiceHistory.md)
 - [GetProjectInvoiceResponse](docs/GetProjectInvoiceResponse.md)
 - [GetSecret](docs/GetSecret.md)
 - [GetServiceServiceWithSpecsResponse](docs/GetServiceServiceWithSpecsResponse.md)
 - [GetServiceSpecResponse](docs/GetServiceSpecResponse.md)
 - [GetServiceSpecsResponse](docs/GetServiceSpecsResponse.md)
 - [GetTransactionResponse](docs/GetTransactionResponse.md)
 - [HeartbeatResponse](docs/HeartbeatResponse.md)
 - [History](docs/History.md)
 - [HttpValidationError](docs/HttpValidationError.md)
 - [KpiResponse](docs/KpiResponse.md)
 - [LineItem](docs/LineItem.md)
 - [LoggingDynamo](docs/LoggingDynamo.md)
 - [MetaChildren](docs/MetaChildren.md)
 - [MetaCreate](docs/MetaCreate.md)
 - [MetaCustom](docs/MetaCustom.md)
 - [MetaDynamo](docs/MetaDynamo.md)
 - [Note](docs/Note.md)
 - [NoteBase](docs/NoteBase.md)
 - [NoteDynamoHistoryResponse](docs/NoteDynamoHistoryResponse.md)
 - [NoteDynamoResponse](docs/NoteDynamoResponse.md)
 - [NoteMeta](docs/NoteMeta.md)
 - [OptionGroup](docs/OptionGroup.md)
 - [Options](docs/Options.md)
 - [Page](docs/Page.md)
 - [Pagination](docs/Pagination.md)
 - [ParticipantCreate](docs/ParticipantCreate.md)
 - [ParticipantUpdate](docs/ParticipantUpdate.md)
 - [ParticipantUserReturn](docs/ParticipantUserReturn.md)
 - [Payment](docs/Payment.md)
 - [PaymentMethodResponse](docs/PaymentMethodResponse.md)
 - [PlaceBase](docs/PlaceBase.md)
 - [PlaceResponse](docs/PlaceResponse.md)
 - [ProductBase](docs/ProductBase.md)
 - [ProductReturn](docs/ProductReturn.md)
 - [ProjectCreditResponse](docs/ProjectCreditResponse.md)
 - [ProjectDb](docs/ProjectDb.md)
 - [ProjectsProjectCreate](docs/ProjectsProjectCreate.md)
 - [ProjectsProjectGet](docs/ProjectsProjectGet.md)
 - [ProjectsProjectMemberDb](docs/ProjectsProjectMemberDb.md)
 - [ProjectsProjectUpdate](docs/ProjectsProjectUpdate.md)
 - [ProjectsProjectUsageDb](docs/ProjectsProjectUsageDb.md)
 - [ProjectsUsageTypeCreate](docs/ProjectsUsageTypeCreate.md)
 - [ProjectsUsageTypeDb](docs/ProjectsUsageTypeDb.md)
 - [ProjectsUsageTypeGet](docs/ProjectsUsageTypeGet.md)
 - [ProjectsUsageTypeUnitPrice](docs/ProjectsUsageTypeUnitPrice.md)
 - [ProjectsUsageTypeUpdate](docs/ProjectsUsageTypeUpdate.md)
 - [ResponseAddmembertoproject](docs/ResponseAddmembertoproject.md)
 - [ResponseArchiveproject](docs/ResponseArchiveproject.md)
 - [ResponseCreatekey](docs/ResponseCreatekey.md)
 - [ResponseCreateprojectcredential](docs/ResponseCreateprojectcredential.md)
 - [ResponseCreateprojectinvoice](docs/ResponseCreateprojectinvoice.md)
 - [ResponseDeletekey](docs/ResponseDeletekey.md)
 - [ResponseDeleteprojectcredential](docs/ResponseDeleteprojectcredential.md)
 - [ResponseDeleteusagetype](docs/ResponseDeleteusagetype.md)
 - [ResponseGeneratetoken](docs/ResponseGeneratetoken.md)
 - [ResponseRemovememberfromproject](docs/ResponseRemovememberfromproject.md)
 - [ResponseRevokeprojectcredit](docs/ResponseRevokeprojectcredit.md)
 - [ResponseUpdateprojectcredential](docs/ResponseUpdateprojectcredential.md)
 - [SaveFact200Response](docs/SaveFact200Response.md)
 - [SecurityCreateToken](docs/SecurityCreateToken.md)
 - [SecurityEncryptionKeyGet](docs/SecurityEncryptionKeyGet.md)
 - [SecurityEncryptionKeyResponse](docs/SecurityEncryptionKeyResponse.md)
 - [SecurityKeyCreate](docs/SecurityKeyCreate.md)
 - [SecurityKeyGet](docs/SecurityKeyGet.md)
 - [SecurityKeyVerify](docs/SecurityKeyVerify.md)
 - [Selection](docs/Selection.md)
 - [ServiceCreate](docs/ServiceCreate.md)
 - [ServiceMessageResponse](docs/ServiceMessageResponse.md)
 - [ServiceResponse](docs/ServiceResponse.md)
 - [ServiceSuperStackMeta](docs/ServiceSuperStackMeta.md)
 - [ServiceSuperStackMetaFaq](docs/ServiceSuperStackMetaFaq.md)
 - [ServiceSuperStackMetaFeature](docs/ServiceSuperStackMetaFeature.md)
 - [ServiceSuperStackMetaGettingStarted](docs/ServiceSuperStackMetaGettingStarted.md)
 - [ServiceSuperStackMetaGettingStartedEndpointTeaser](docs/ServiceSuperStackMetaGettingStartedEndpointTeaser.md)
 - [ServiceSuperStackMetaUseCase](docs/ServiceSuperStackMetaUseCase.md)
 - [Slugger](docs/Slugger.md)
 - [StaffCreate](docs/StaffCreate.md)
 - [StaffDb](docs/StaffDb.md)
 - [StaffResponse](docs/StaffResponse.md)
 - [StatsVitalsResponse](docs/StatsVitalsResponse.md)
 - [StripeAccountResponse](docs/StripeAccountResponse.md)
 - [StripeCustomerSecretResponse](docs/StripeCustomerSecretResponse.md)
 - [TagBase](docs/TagBase.md)
 - [TagDb](docs/TagDb.md)
 - [Tax](docs/Tax.md)
 - [TicketResponse](docs/TicketResponse.md)
 - [TicketsResponse](docs/TicketsResponse.md)
 - [TouchMeta200Response](docs/TouchMeta200Response.md)
 - [UpdateField200Response](docs/UpdateField200Response.md)
 - [UpdateFile200Response](docs/UpdateFile200Response.md)
 - [UpdateMeta200Response](docs/UpdateMeta200Response.md)
 - [UpdateNote200Response](docs/UpdateNote200Response.md)
 - [UpdateProjectCredentialRequest](docs/UpdateProjectCredentialRequest.md)
 - [UpdateReview](docs/UpdateReview.md)
 - [User](docs/User.md)
 - [UserConfirmation](docs/UserConfirmation.md)
 - [UserEmail](docs/UserEmail.md)
 - [UserFlags](docs/UserFlags.md)
 - [UserLogin](docs/UserLogin.md)
 - [UserLoginReturn](docs/UserLoginReturn.md)
 - [UserPasswordReset](docs/UserPasswordReset.md)
 - [UserPasswordResetConfirmation](docs/UserPasswordResetConfirmation.md)
 - [UserResponse](docs/UserResponse.md)
 - [UserSignup](docs/UserSignup.md)
 - [UserSignupReturn](docs/UserSignupReturn.md)
 - [UserTokenReturn](docs/UserTokenReturn.md)
 - [UserValidations](docs/UserValidations.md)
 - [ValidationError](docs/ValidationError.md)
 - [Validations](docs/Validations.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@ehelply.com

