use anyhow::{Result, Ok};
use serde_json::{json, Value};

use crate::crawler::{query, DrugProductData};

pub mod crawler;

fn main() -> Result<()> {
    // let input = "100";
    // crawler::query(input)
    let product_data = r#"
    {
        "draw": 2,
        "recordsTotal": 6549,
        "recordsFiltered": 6549,
        "data": [
            {
                "product_id": 4191,
                "ingredient_id": 191,
                "manufacturer_id": 26,
                "product_name": "4 Oral Powder",
                "form_id": "111",
                "strength": "20.5 g",
                "NAFDAC": "B4-5544",
                "product_category_id": 1,
                "marketing_category_id": 2,
                "applicant_id": 179,
                "approval_date": "2021-03-01",
                "expiry_date": "2026-02-28",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "Powder for oral solution in 20.5 g sachet.\r\nEach sachet contains: Dextrose anhydrous 13.5 g, Sodium chloride 2.6 g, Sodium citrate 2.9 g, Potassium chloride 1.5 g",
                "pack_size": "",
                "biosimilar": null,
                "atc": "A07CA",
                "created_at": "2022-10-24T11:27:41.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 191,
                    "ingredient_name": "Oral Rehydration Salts",
                    "synonym": "ORS",
                    "updated_at": "2022-07-01T15:09:54.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 111,
                    "name": "Powder",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-12T04:09:05.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 179,
                    "name": "Geneith Pharmaceuticals Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-17T04:26:24.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 1
            },
            {
                "product_id": 2117,
                "ingredient_id": 372,
                "manufacturer_id": 6,
                "product_name": "4.3% Dextrose &amp; Normal Saline Infusion",
                "form_id": "114",
                "strength": "4.3%; 0.9%",
                "NAFDAC": "A11-0238",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 27,
                "approval_date": "2019-02-09",
                "expiry_date": "2023-02-08",
                "route_id": 12,
                "smpc": "",
                "country_id": null,
                "product_description": "Solution for Infusion",
                "pack_size": "",
                "biosimilar": null,
                "atc": "B05BB02",
                "created_at": "2022-07-13T04:03:47.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 372,
                    "ingredient_name": "Glucose; Sodium Chloride",
                    "synonym": "Dextrose; Sodium chloride",
                    "updated_at": "2022-06-22T15:02:26.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 114,
                    "name": "Solution for infusion",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-16T07:25:53.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 27,
                    "name": "Fidson Healthcare PLC",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-10-11T02:00:58.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 12,
                    "name": "Intravenous",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:04:25.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 2
            },
            {
                "product_id": 2118,
                "ingredient_id": 372,
                "manufacturer_id": 6,
                "product_name": "5% Dextrose &amp; Normal Saline Infusion",
                "form_id": "114",
                "strength": "5%; 0.9%",
                "NAFDAC": "A11-0236",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 27,
                "approval_date": "2019-02-09",
                "expiry_date": "2023-02-08",
                "route_id": 12,
                "smpc": "",
                "country_id": null,
                "product_description": "Solution for Infusion",
                "pack_size": "",
                "biosimilar": null,
                "atc": "B05BB02",
                "created_at": "2022-07-13T04:03:57.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 372,
                    "ingredient_name": "Glucose; Sodium Chloride",
                    "synonym": "Dextrose; Sodium chloride",
                    "updated_at": "2022-06-22T15:02:26.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 114,
                    "name": "Solution for infusion",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-16T07:25:53.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 27,
                    "name": "Fidson Healthcare PLC",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-10-11T02:00:58.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 12,
                    "name": "Intravenous",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:04:25.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 3
            },
            {
                "product_id": 2119,
                "ingredient_id": 638,
                "manufacturer_id": 6,
                "product_name": "5% Dextrose Infusion",
                "form_id": "114",
                "strength": "5%",
                "NAFDAC": "A11-0240",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 27,
                "approval_date": "2019-02-09",
                "expiry_date": "2023-02-08",
                "route_id": 12,
                "smpc": "",
                "country_id": null,
                "product_description": "Solution for Infusion",
                "pack_size": "",
                "biosimilar": null,
                "atc": "B05BA03",
                "created_at": "2022-07-13T04:04:10.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 638,
                    "ingredient_name": "Glucose",
                    "synonym": "Dextrose",
                    "updated_at": "2022-05-09T02:52:49.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 114,
                    "name": "Solution for infusion",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-16T07:25:53.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 27,
                    "name": "Fidson Healthcare PLC",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-10-11T02:00:58.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 12,
                    "name": "Intravenous",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:04:25.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 4
            },
            {
                "product_id": 2120,
                "ingredient_id": 638,
                "manufacturer_id": 130,
                "product_name": "50% w\/v Glucose Intravenous Infusion",
                "form_id": "114",
                "strength": "50%",
                "NAFDAC": "A4-7253",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 162,
                "approval_date": "2018-04-04",
                "expiry_date": "2023-04-03",
                "route_id": 12,
                "smpc": "",
                "country_id": null,
                "product_description": "Solution for Infusion",
                "pack_size": "100 mL",
                "biosimilar": null,
                "atc": "B05BA03",
                "created_at": "2022-07-13T04:14:53.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 638,
                    "ingredient_name": "Glucose",
                    "synonym": "Dextrose",
                    "updated_at": "2022-05-09T02:52:49.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 114,
                    "name": "Solution for infusion",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-16T07:25:53.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 162,
                    "name": "Drugfield Pharmaceuticals Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-16T02:19:35.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 12,
                    "name": "Intravenous",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:04:25.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 5
            },
            {
                "product_id": 2638,
                "ingredient_id": 167,
                "manufacturer_id": 535,
                "product_name": "A-Mag Antacid Suspension",
                "form_id": "102",
                "strength": "250 mg\/5 mL; 250 mg\/5 mL; 250 mg\/5 mL",
                "NAFDAC": "04-4335",
                "product_category_id": 1,
                "marketing_category_id": 2,
                "applicant_id": 705,
                "approval_date": "2019-01-31",
                "expiry_date": "2024-01-30",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "Each 5 mL suspension contains: Magnesium Trisilicate 250 mg, Light Magnesium Carbonate 250 mg, Sodium Bicarbonate 250 mg",
                "pack_size": "",
                "biosimilar": null,
                "atc": "A02AH",
                "created_at": "2022-10-08T01:37:59.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 167,
                    "ingredient_name": "Magnesium trisilicate; Magnesium carbonate; Sodium bicarbonate",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 102,
                    "name": "Suspension",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-05T08:04:42.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 705,
                    "name": "Pal Pharmaceutical Industries Ltd",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-07-08T15:26:54.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 6
            },
            {
                "product_id": 643,
                "ingredient_id": 56,
                "manufacturer_id": 259,
                "product_name": "Aarodol 100 Capsule",
                "form_id": "2",
                "strength": "100 mg",
                "NAFDAC": "B4-3103",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 360,
                "approval_date": "2021-05-27",
                "expiry_date": "2026-05-26",
                "route_id": 1,
                "smpc": "https:\/\/41.138.161.61\/admin\/uploadImage\/smpc_files\/2023\/05\/27\/20230527df555986-70c4-5bb4-b487-1cc720ba5541.pdf",
                "country_id": null,
                "product_description": "Each capsule contains: Tramadol Hydrochloride 100 mg",
                "pack_size": "",
                "biosimilar": null,
                "atc": "N02AX02",
                "created_at": "2022-06-23T04:22:26.000000Z",
                "updated_at": "2023-05-27T11:11:32.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 56,
                    "ingredient_name": "Tramadol Hydrochloride",
                    "synonym": "",
                    "updated_at": "2022-06-11T01:16:39.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 2,
                    "name": "Capsule",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 360,
                    "name": "Ugolab Productions Nigeria Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-22T02:54:05.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 7
            },
            {
                "product_id": 644,
                "ingredient_id": 56,
                "manufacturer_id": 259,
                "product_name": "Aarodol 50 Capsule",
                "form_id": "2",
                "strength": "50 mg",
                "NAFDAC": "B4-3102",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 360,
                "approval_date": "2021-05-27",
                "expiry_date": "2026-05-26",
                "route_id": 1,
                "smpc": "https:\/\/41.138.161.61\/admin\/uploadImage\/smpc_files\/2023\/05\/27\/20230527fd693435-81c1-5ed9-9b5d-28e4f8ecdd53.pdf",
                "country_id": null,
                "product_description": "Each capsule contains: Tramadol Hydrochloride 50 mg",
                "pack_size": "",
                "biosimilar": null,
                "atc": "N02AX02",
                "created_at": "2022-06-23T04:23:36.000000Z",
                "updated_at": "2023-05-27T11:13:57.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 56,
                    "ingredient_name": "Tramadol Hydrochloride",
                    "synonym": "",
                    "updated_at": "2022-06-11T01:16:39.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 2,
                    "name": "Capsule",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 360,
                    "name": "Ugolab Productions Nigeria Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-22T02:54:05.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 8
            },
            {
                "product_id": 2639,
                "ingredient_id": 601,
                "manufacturer_id": 184,
                "product_name": "Abacavir &amp; Lamivudine Tablets 600\/300 mg",
                "form_id": "1",
                "strength": "600 mg; 300 mg",
                "NAFDAC": "A4-100079",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 276,
                "approval_date": "2021-08-26",
                "expiry_date": "2026-08-25",
                "route_id": 1,
                "smpc": "https:\/\/41.138.161.61\/admin\/uploadImage\/smpc_files\/2023\/05\/27\/2023052755eea184-ca28-5dbf-963a-6350e3f63e18.pdf",
                "country_id": null,
                "product_description": "Film-coated tablet",
                "pack_size": "",
                "biosimilar": null,
                "atc": "J05AR02",
                "created_at": "2022-10-08T01:40:28.000000Z",
                "updated_at": "2023-05-27T11:19:07.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 601,
                    "ingredient_name": "Abacavir; Lamivudine",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 1,
                    "name": "Tablet",
                    "updated_at": null,
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 276,
                    "name": "Ranbaxy Nigeria Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-19T19:31:38.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 9
            },
            {
                "product_id": 2640,
                "ingredient_id": 601,
                "manufacturer_id": 136,
                "product_name": "Abacavir (As Sulfate) And Lamivudine Dispersible Tablets 120 mg\/60 mg",
                "form_id": "13",
                "strength": "120 mg; 60 mg",
                "NAFDAC": "C4-1181",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 193,
                "approval_date": "2020-11-10",
                "expiry_date": "2025-11-09",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "Dispersible tablet",
                "pack_size": "",
                "biosimilar": null,
                "atc": "J05AR02",
                "created_at": "2022-10-08T01:42:33.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 601,
                    "ingredient_name": "Abacavir; Lamivudine",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 13,
                    "name": "Dispersible tablet",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:14:45.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 193,
                    "name": "Healthline Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 10
            },
            {
                "product_id": 1680,
                "ingredient_id": 601,
                "manufacturer_id": 62,
                "product_name": "Abacavir and Lamivudine Tablets for Oral Suspension",
                "form_id": "102",
                "strength": "120 mg; 60 mg",
                "NAFDAC": "B4-8619",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 271,
                "approval_date": "2018-06-14",
                "expiry_date": "2023-06-13",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "Tablets for Oral Suspension\r\nEach tablet contains: Abacavir 120 mg, Lamivudine 60 mg",
                "pack_size": "",
                "biosimilar": null,
                "atc": "J05AR02",
                "created_at": "2022-07-06T03:31:52.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 601,
                    "ingredient_name": "Abacavir; Lamivudine",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 102,
                    "name": "Suspension",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-05T08:04:42.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 271,
                    "name": "Phillips Pharmaceuticals Nigeria Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-18T02:27:32.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 11
            },
            {
                "product_id": 2641,
                "ingredient_id": 601,
                "manufacturer_id": 147,
                "product_name": "Abacavir Sulfate &amp; Lamivudine Tablet",
                "form_id": "1",
                "strength": "600 mg; 300 mg",
                "NAFDAC": "C4-1202",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 1063,
                "approval_date": "2020-11-10",
                "expiry_date": "2025-11-09",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "",
                "pack_size": "",
                "biosimilar": null,
                "atc": "J05AR02",
                "created_at": "2022-10-08T01:46:07.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 601,
                    "ingredient_name": "Abacavir; Lamivudine",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 1,
                    "name": "Tablet",
                    "updated_at": null,
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 1063,
                    "name": "Hetero Labs Nigeria Limited",
                    "address": null,
                    "created_at": "2022-06-16T08:50:31.000000Z",
                    "updated_at": "2022-10-08T00:46:52.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 12
            },
            {
                "product_id": 2642,
                "ingredient_id": 601,
                "manufacturer_id": 136,
                "product_name": "Abacavir Sulfate And Lamivudine Dispersible Tablets 60 mg\/30 mg",
                "form_id": "13",
                "strength": "60 mg; 30 mg",
                "NAFDAC": "C4-1180",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 193,
                "approval_date": "2020-11-10",
                "expiry_date": "2025-11-09",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "Dispersible tablet",
                "pack_size": "",
                "biosimilar": null,
                "atc": "J05AR02",
                "created_at": "2022-10-08T01:49:46.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 601,
                    "ingredient_name": "Abacavir; Lamivudine",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 13,
                    "name": "Dispersible tablet",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:14:45.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 193,
                    "name": "Healthline Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 13
            },
            {
                "product_id": 1834,
                "ingredient_id": 585,
                "manufacturer_id": 62,
                "product_name": "Abacavir Sulfate Tablets For Oral Suspension",
                "form_id": "102",
                "strength": "60 mg",
                "NAFDAC": "A4-5531",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 271,
                "approval_date": "2018-04-04",
                "expiry_date": "2023-04-03",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "Tablets for Oral Suspension\r\nEach tablet contains: Abacavir Sulfate USP 60 mg",
                "pack_size": "",
                "biosimilar": null,
                "atc": "J05AF06",
                "created_at": "2022-07-07T09:47:09.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 585,
                    "ingredient_name": "Abacavir Sulfate",
                    "synonym": "ABC",
                    "updated_at": "2022-05-08T14:38:54.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 102,
                    "name": "Suspension",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-05T08:04:42.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 271,
                    "name": "Phillips Pharmaceuticals Nigeria Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-18T02:27:32.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 14
            },
            {
                "product_id": 2635,
                "ingredient_id": 585,
                "manufacturer_id": 147,
                "product_name": "Abacavir Tablets",
                "form_id": "1",
                "strength": "300 mg",
                "NAFDAC": "B4-3826",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 1063,
                "approval_date": "2020-11-10",
                "expiry_date": "2025-11-09",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "",
                "pack_size": "",
                "biosimilar": null,
                "atc": "J05AF06",
                "created_at": "2022-07-26T01:35:04.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 585,
                    "ingredient_name": "Abacavir Sulfate",
                    "synonym": "ABC",
                    "updated_at": "2022-05-08T14:38:54.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 1,
                    "name": "Tablet",
                    "updated_at": null,
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 1063,
                    "name": "Hetero Labs Nigeria Limited",
                    "address": null,
                    "created_at": "2022-06-16T08:50:31.000000Z",
                    "updated_at": "2022-10-08T00:46:52.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 15
            },
            {
                "product_id": 5494,
                "ingredient_id": 1688,
                "manufacturer_id": 739,
                "product_name": "ABDOE-BISMUTH",
                "form_id": "103",
                "strength": "262mg",
                "NAFDAC": "A11-100490",
                "product_category_id": 1,
                "marketing_category_id": 2,
                "applicant_id": 1211,
                "approval_date": "2023-04-26",
                "expiry_date": "2028-04-25",
                "route_id": 1,
                "smpc": "https:\/\/41.138.161.61\/admin\/uploadImage\/smpc_files\/2023\/09\/25\/2023092532c1e9af-3536-5746-9a8c-c5161dbf4546.pdf",
                "country_id": null,
                "product_description": "4\tNF-PP-294351\tABDOE-BISMUTH\tBISMUTH SUBSALICYLATE\tBISMUTH SUBSALICYLATE 262MG\tA11-100490\tDrugs\tALIMENTARY TRACT AND METABOLISM\tNigerian Products\t150ML\tOTC\t11\/8\/2022\tJEHYSON HEALTHCARE LTD, LAGOS ABEOKUTA EXPRESSWAY LAGOS LAGOS\t8066703444\tjehysonhealthcare@gmail.com\tJEHYSON HEALTHCARE LIMITED, JEHYSON CRESCENT, 78KM LAGOS-ABEOKUTA EXP-WAY, APOMU, EWEKORO LGA, OGUN STATE, Nigeria\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n",
                "pack_size": "150ML",
                "biosimilar": null,
                "atc": "A07BB",
                "created_at": "2023-09-26T11:15:50.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 1688,
                    "ingredient_name": "BISMUTH SUBSALICYLATE",
                    "synonym": "",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2023-09-25T10:21:25.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 103,
                    "name": "Syrup",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-05T08:29:30.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 1211,
                    "name": "JEHYSON HEALTHCARE LTD",
                    "address": null,
                    "created_at": "2023-09-25T10:25:11.000000Z",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 16
            },
            {
                "product_id": 764,
                "ingredient_id": 35,
                "manufacturer_id": 67,
                "product_name": "Ablept-300 Capsules",
                "form_id": "2",
                "strength": "300 mg",
                "NAFDAC": "B4-9460",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 230,
                "approval_date": "2018-12-20",
                "expiry_date": "2023-12-19",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "",
                "pack_size": "",
                "biosimilar": null,
                "atc": "N03AX12",
                "created_at": "2022-06-25T01:01:14.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 35,
                    "ingredient_name": "Gabapentin",
                    "synonym": "",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 2,
                    "name": "Capsule",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 230,
                    "name": "Micronova Pharmaceutical Ind. Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-10-17T02:06:47.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 17
            },
            {
                "product_id": 2643,
                "ingredient_id": 1141,
                "manufacturer_id": 526,
                "product_name": "Aboniki Balm",
                "form_id": "4",
                "strength": "4.25%; 6.1%; 5.04%; 2.3%",
                "NAFDAC": "04-5773",
                "product_category_id": 1,
                "marketing_category_id": 2,
                "applicant_id": 911,
                "approval_date": "2019-02-27",
                "expiry_date": "2023-02-26",
                "route_id": 21,
                "smpc": "",
                "country_id": null,
                "product_description": "Menthol 6.10%, Eucalyptus 2.30%, Methyl Salicylate 4.25%, Camphor 5.04%",
                "pack_size": "",
                "biosimilar": null,
                "atc": " M02AC",
                "created_at": "2022-10-08T01:59:11.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 1141,
                    "ingredient_name": "Methyl salicylate; Menthol; Camphor; Eucalyptus oil",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 4,
                    "name": "Cream",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-05T03:39:22.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 911,
                    "name": "JC Udeozor &amp; Sons Global Industrial Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-10-08T01:00:29.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 21,
                    "name": "Topical",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:06:24.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 18
            },
            {
                "product_id": 5503,
                "ingredient_id": 551,
                "manufacturer_id": 661,
                "product_name": "ABTHER 150 INJECTION",
                "form_id": "40",
                "strength": "150 mg\/2 mL",
                "NAFDAC": "B4-7301",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 189,
                "approval_date": "2023-03-28",
                "expiry_date": "2028-03-27",
                "route_id": 29,
                "smpc": "",
                "country_id": null,
                "product_description": "103\tRNW-PP-271339\tABTHER 150 INJECTION\ta-\u00df ARTEETHER 150MG\/2ML\ta-\u00df ARTEETHER 150MG\/2ML, ARACHIS OIL, BUTYLATED HYDROXYTOLUENE\tB4-7301\tDrugs\tANTIPARASITIC PRODUCTS, INSECTICIDES AND REPELLENTS\tImported Products\t3X2ML\tPOM 1\t6\/22\/2022\tGREENLIFE PHARMACEUTICALS LIMITED, 35A, ASSOCIATION AVENUE, ILUPEJU, LAGOS\t17378995\tgreenlife.regulatory@yahoo.com\tINDASI LIFESCIENCE PVT LTD, PLOT NO. 73 TO 76, SILVER INDUSTRIAL ESTATE, BHIMPORE, DAMAN-396210, INDIA, India\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n",
                "pack_size": "3X2ML",
                "biosimilar": null,
                "atc": "P01BE",
                "created_at": "2023-09-26T03:06:31.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 551,
                    "ingredient_name": "\u03b1-\u03b2 Arteether",
                    "synonym": "",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 40,
                    "name": "Injection",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:23:27.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 189,
                    "name": "Greenlife Pharmaceuticals Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-21T03:46:11.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 29,
                    "name": "Intravenous; Intramuscular",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-06-11T04:58:09.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 19
            },
            {
                "product_id": 5898,
                "ingredient_id": 551,
                "manufacturer_id": 661,
                "product_name": "ABTHER 225 INJECTION",
                "form_id": "40",
                "strength": "225 mg\/3 mL",
                "NAFDAC": "B4-7300",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 189,
                "approval_date": "2023-04-26",
                "expiry_date": "2028-04-25",
                "route_id": 9,
                "smpc": "https:\/\/41.138.161.61\/admin\/uploadImage\/smpc_files\/2023\/09\/28\/202309280ca84257-a542-5942-8aed-d6a781c8146c.pdf",
                "country_id": null,
                "product_description": "ABTHER 225 INJECTION\ta-\u00df ARTEETHER 225MG\/3ML\ta-\u00df ARTEETHER 225MG\/3ML, ARACHIS OIL, BUTYLATED HYDROXYTOLUENE\tB4-7300\tDrugs\tImported Products\t3X3ML\tPrescription Only Medicine (POM)\t22\/06\/2022\tGREENLIFE PHARMACEUTICALS LIMITED, 35A, ASSOCIATION AVENUE, ILUPEJU, LAGOS\t17378995\tgreenlife.regulatory@yahoo.com\tINDASI LIFESCIENCE PVT LTD, PLOT NO. 73 TO 76, SILVER INDUSTRIAL ESTATE, BHIMPORE, DAMAN-396210, INDIA, India\r\n\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\r\n",
                "pack_size": "",
                "biosimilar": null,
                "atc": "P01BE01",
                "created_at": "2023-10-03T02:02:28.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 551,
                    "ingredient_name": "\u03b1-\u03b2 Arteether",
                    "synonym": "",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 40,
                    "name": "Injection",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:23:27.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 189,
                    "name": "Greenlife Pharmaceuticals Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-21T03:46:11.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 9,
                    "name": "Intramuscular",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:03:50.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 20
            },
            {
                "product_id": 1073,
                "ingredient_id": 571,
                "manufacturer_id": 335,
                "product_name": "AC Mebezole Tablet",
                "form_id": "1",
                "strength": "100 mg",
                "NAFDAC": "A11-0762",
                "product_category_id": 1,
                "marketing_category_id": 2,
                "applicant_id": 355,
                "approval_date": "2018-10-03",
                "expiry_date": "2023-10-02",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "",
                "pack_size": "",
                "biosimilar": null,
                "atc": "P02CA01",
                "created_at": "2022-06-30T07:40:14.000000Z",
                "updated_at": "2022-10-15T03:11:42.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 571,
                    "ingredient_name": "Mebendazole",
                    "synonym": "",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 1,
                    "name": "Tablet",
                    "updated_at": null,
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 355,
                    "name": "A. C. Drugs Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-30T18:38:56.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 21
            },
            {
                "product_id": 2637,
                "ingredient_id": 56,
                "manufacturer_id": 335,
                "product_name": "AC Tramadol Capsules",
                "form_id": "2",
                "strength": "100 mg",
                "NAFDAC": "A4-7559",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 355,
                "approval_date": "2019-08-29",
                "expiry_date": "2024-08-28",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "Each capsule contains: Tramadol Hydrochloride 100 mg",
                "pack_size": "",
                "biosimilar": null,
                "atc": "N02AX02",
                "created_at": "2022-08-27T05:00:08.000000Z",
                "updated_at": "2022-08-27T04:01:49.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 56,
                    "ingredient_name": "Tramadol Hydrochloride",
                    "synonym": "",
                    "updated_at": "2022-06-11T01:16:39.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 2,
                    "name": "Capsule",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 355,
                    "name": "A. C. Drugs Limited",
                    "address": null,
                    "created_at": "2022-05-07T02:03:01.000000Z",
                    "updated_at": "2022-06-30T18:38:56.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 22
            },
            {
                "product_id": 4666,
                "ingredient_id": 4,
                "manufacturer_id": 696,
                "product_name": "AC-Diazepam",
                "form_id": "1",
                "strength": "5 mg",
                "NAFDAC": "A11-100499",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 1189,
                "approval_date": "2023-05-25",
                "expiry_date": "2028-05-24",
                "route_id": 1,
                "smpc": "https:\/\/41.138.161.61\/admin\/uploadImage\/smpc_files\/2023\/10\/02\/20231002134fc447-8c8d-5635-a26b-cdacd666c753.pdf",
                "country_id": null,
                "product_description": "56\tNF-PP-201002\tAC-Diazepam\tDiazepam\tDiazepam 5mg\t\tDrugs\tNigerian Products\t10*10PACKS\tPOM 1\t3\/9\/2021\tA.C. DRUGS LTD, 4 ALOR ROAD THINKERS CORNER, ENUGU\t80334641\tacdrugslimited@yahoo.com\tAC DRUGS LIMITED, 4, ALOR ROAD, L\/O EDWARD NNANJI, ENUGU, ENUGU STATE., Nigeria\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n",
                "pack_size": "10*10PACKS",
                "biosimilar": null,
                "atc": "N05BA01",
                "created_at": "2023-08-23T11:14:16.000000Z",
                "updated_at": "2023-10-02T13:55:17.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 4,
                    "ingredient_name": "Diazepam",
                    "synonym": "",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 1,
                    "name": "Tablet",
                    "updated_at": null,
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 1189,
                    "name": "AC DRUGS LIMITED",
                    "address": null,
                    "created_at": "2023-08-04T09:55:38.000000Z",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 23
            },
            {
                "product_id": 5495,
                "ingredient_id": 5,
                "manufacturer_id": 696,
                "product_name": "AC-Flunitrazepam",
                "form_id": "1",
                "strength": "1 mg",
                "NAFDAC": "A11-100485",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 1189,
                "approval_date": "2023-04-26",
                "expiry_date": "2028-04-25",
                "route_id": 1,
                "smpc": "",
                "country_id": null,
                "product_description": "22\tNF-PP-201010\tAC-Flunitrazepam\tFlunitrazepam\tFlunitrazepam 1mg\tA11-100485\tDrugs\tNERVOUS SYSTEM\tNigerian Products\t10*3PACKS\tPOM 1\t3\/9\/2021\tA.C. DRUGS LTD, 4 ALOR ROAD THINKERS CORNER, ENUGU\t80334641\tacdrugslimited@yahoo.com\tAC DRUGS LIMITED, 4, ALOR ROAD, L\/O EDWARD NNANJI, ENUGU, ENUGU STATE., Nigeria\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\r\n",
                "pack_size": "10 X 3PACKS",
                "biosimilar": null,
                "atc": "N05CD03",
                "created_at": "2023-09-26T11:17:42.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 5,
                    "ingredient_name": "Flunitrazepam",
                    "synonym": "",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 1,
                    "name": "Tablet",
                    "updated_at": null,
                    "created_at": "2022-03-19T06:40:15.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 1189,
                    "name": "AC DRUGS LIMITED",
                    "address": null,
                    "created_at": "2023-08-04T09:55:38.000000Z",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 1,
                    "name": "Oral",
                    "updated_at": null,
                    "created_at": null,
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 24
            },
            {
                "product_id": 3949,
                "ingredient_id": 1106,
                "manufacturer_id": 18,
                "product_name": "Accesslife Clotrimazole, Betamethasone and Neomycin Cream",
                "form_id": "4",
                "strength": "0.05%; 1%; 0.5%",
                "NAFDAC": "A4-100125",
                "product_category_id": 1,
                "marketing_category_id": 1,
                "applicant_id": 1053,
                "approval_date": "2021-10-28",
                "expiry_date": "2026-10-27",
                "route_id": 21,
                "smpc": "",
                "country_id": null,
                "product_description": "Clotrimazole 1% w\/w, Betamethasone 0.05% w\/w, and Neomycin 0.5% w\/w",
                "pack_size": "",
                "biosimilar": null,
                "atc": "D07CC01",
                "created_at": "2022-10-20T01:41:57.000000Z",
                "updated_at": "-000001-11-30T00:00:00.000000Z",
                "deleted_at": null,
                "ingredient": {
                    "ingredient_id": 1106,
                    "ingredient_name": "Betamethasone; Clotrimazole; Neomycin",
                    "synonym": "",
                    "updated_at": "2022-05-17T11:51:11.000000Z",
                    "created_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "form": {
                    "id": 4,
                    "name": "Cream",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-05T03:39:22.000000Z",
                    "deleted_at": null
                },
                "applicant": {
                    "id": 1053,
                    "name": "Accesslife Pharmaceutical Nig. Ltd",
                    "address": null,
                    "created_at": "2022-06-06T10:29:15.000000Z",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "deleted_at": null
                },
                "route": {
                    "id": 21,
                    "name": "Topical",
                    "updated_at": "-000001-11-30T00:00:00.000000Z",
                    "created_at": "2022-05-06T03:06:24.000000Z",
                    "deleted_at": null
                },
                "status": "Active",
                "DT_RowIndex": 25
            }
        ],
        "queries": [
            {
                "query": "select count(*) as aggregate from `products` where `products`.`deleted_at` is null",
                "bindings": [],
                "time": 7.94000000000000039079850466805510222911834716796875
            },
            {
                "query": "select * from `products` where `products`.`deleted_at` is null order by `product_name` asc limit 25 offset 0",
                "bindings": [],
                "time": 23.28999999999999914734871708787977695465087890625
            },
            {
                "query": "select * from `ingredients` where `ingredients`.`ingredient_id` in (4, 5, 35, 56, 167, 191, 372, 551, 571, 585, 601, 638, 1106, 1141, 1688) and `ingredients`.`deleted_at` is null",
                "bindings": [],
                "time": 0.68000000000000004884981308350688777863979339599609375
            },
            {
                "query": "select * from `forms` where `forms`.`id` in (1, 2, 4, 13, 40, 102, 103, 111, 114) and `forms`.`deleted_at` is null",
                "bindings": [],
                "time": 0.419999999999999984456877655247808434069156646728515625
            },
            {
                "query": "select * from `applicants` where `applicants`.`id` in (27, 162, 179, 189, 193, 230, 271, 276, 355, 360, 705, 911, 1053, 1063, 1189, 1211) and `applicants`.`deleted_at` is null",
                "bindings": [],
                "time": 0.450000000000000011102230246251565404236316680908203125
            },
            {
                "query": "select * from `routes` where `routes`.`id` in (1, 9, 12, 21, 29) and `routes`.`deleted_at` is null",
                "bindings": [],
                "time": 0.38000000000000000444089209850062616169452667236328125
            }
        ],
        "input": {
            "draw": "2",
            "columns": [
                {
                    "data": "product_name",
                    "name": "product_name",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "ingredient.ingredient_name",
                    "name": "ingredient.ingredient_name",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "ingredient.synonym",
                    "name": "ingredient.synonym",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "NAFDAC",
                    "name": "NAFDAC",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "form.name",
                    "name": "form.name",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "route.name",
                    "name": "route.name",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "strength",
                    "name": "strength",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "applicant.name",
                    "name": "applicant.name",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "approval_date",
                    "name": "approval_date",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                },
                {
                    "data": "status",
                    "name": "status",
                    "searchable": "true",
                    "orderable": "true",
                    "search": {
                        "value": null,
                        "regex": "false"
                    }
                }
            ],
            "order": [
                {
                    "column": "0",
                    "dir": "asc"
                }
            ],
            "start": "0",
            "length": "25",
            "search": {
                "value": null,
                "regex": "false"
            },
            "_": "1703974467158"
        }
    }
    "#;

    // Parse the string of data into serde_json::Value.
    let v: DrugProductData = serde_json::from_str(product_data)?;
    println!("v: {:?}", v);

    Ok(())
}
