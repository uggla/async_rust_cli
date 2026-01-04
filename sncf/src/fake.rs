pub fn api_base() -> String {
    r#"{
  "links": [
    {
      "href": "https://api.sncf.com/v1/coverage/",
      "templated": false,
      "rel": "coverage",
      "type": "coverage",
      "title": "Coverage of navitia"
    },
    {
      "href": "https://api.sncf.com/v1/coord/0.0%3B0.0/",
      "templated": true,
      "rel": "coord",
      "type": "coord",
      "title": "Inverted geocoding for a given coordinate"
    },
    {
      "href": "https://api.sncf.com/v1/journeys",
      "templated": false,
      "rel": "journeys",
      "type": "journeys",
      "title": "Compute journeys"
    },
    {
      "href": "https://api.sncf.com/v1/places",
      "templated": false,
      "rel": "places",
      "type": "places",
      "title": "Autocomplete api"
    }
  ]
}"#
    .to_string()
}

pub fn places() -> String {
    r#"{
  "feed_publishers": [
    {
      "id": "sncf",
      "name": "SNCF PIV Production",
      "url": "",
      "license": "Private (unspecified)"
    },
    {
      "id": "SNCF:sncf-piv",
      "name": "SNCF PIV Production",
      "url": "",
      "license": "Private (unspecified)"
    }
  ],
  "disruptions": [],
  "places": [
    {
      "id": "admin:fr:38185",
      "name": "Grenoble (38000-38100)",
      "quality": 80,
      "administrative_region": {
        "id": "admin:fr:38185",
        "name": "Grenoble",
        "level": 8,
        "zip_code": "38000;38100",
        "label": "Grenoble (38000-38100)",
        "insee": "38185",
        "coord": {
          "lon": "5.7357819",
          "lat": "45.1875602"
        }
      },
      "embedded_type": "administrative_region"
    },
    {
      "id": "stop_area:SNCF:87747006",
      "name": "Grenoble (Grenoble)",
      "quality": 100,
      "stop_area": {
        "id": "stop_area:SNCF:87747006",
        "name": "Grenoble",
        "codes": [
          {
            "type": "source",
            "value": "87747006"
          },
          {
            "type": "uic",
            "value": "87747006"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble (Grenoble)",
        "coord": {
          "lon": "5.714548",
          "lat": "45.191491"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38185",
            "name": "Grenoble",
            "level": 8,
            "zip_code": "38000;38100",
            "label": "Grenoble (38000-38100)",
            "insee": "38185",
            "coord": {
              "lon": "5.7357819",
              "lat": "45.1875602"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    },
    {
      "id": "stop_area:SNCF:87747402",
      "name": "Grenoble Universités - Gières (Gières)",
      "quality": 80,
      "stop_area": {
        "id": "stop_area:SNCF:87747402",
        "name": "Grenoble Universités - Gières",
        "codes": [
          {
            "type": "source",
            "value": "87747402"
          },
          {
            "type": "uic",
            "value": "87747402"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble Universités - Gières (Gières)",
        "coord": {
          "lon": "5.784731",
          "lat": "45.184971"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38179",
            "name": "Gières",
            "level": 8,
            "zip_code": "38610",
            "label": "Gières (38610)",
            "insee": "38179",
            "coord": {
              "lon": "5.790419399999999",
              "lat": "45.1798245"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    },
    {
      "id": "stop_area:SNCF:87697482",
      "name": "Grenoble - Champollion (Grenoble)",
      "quality": 90,
      "stop_area": {
        "id": "stop_area:SNCF:87697482",
        "name": "Grenoble - Champollion",
        "codes": [
          {
            "type": "source",
            "value": "87697482"
          },
          {
            "type": "uic",
            "value": "87697482"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble - Champollion (Grenoble)",
        "coord": {
          "lon": "5.725304",
          "lat": "45.186336"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38185",
            "name": "Grenoble",
            "level": 8,
            "zip_code": "38000;38100",
            "label": "Grenoble (38000-38100)",
            "insee": "38185",
            "coord": {
              "lon": "5.7357819",
              "lat": "45.1875602"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    },
    {
      "id": "stop_area:SNCF:87697490",
      "name": "Grenoble Europole (Grenoble)",
      "quality": 90,
      "stop_area": {
        "id": "stop_area:SNCF:87697490",
        "name": "Grenoble Europole",
        "codes": [
          {
            "type": "source",
            "value": "87697490"
          },
          {
            "type": "uic",
            "value": "87697490"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble Europole (Grenoble)",
        "coord": {
          "lon": "5.713017",
          "lat": "45.191241"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38185",
            "name": "Grenoble",
            "level": 8,
            "zip_code": "38000;38100",
            "label": "Grenoble (38000-38100)",
            "insee": "38185",
            "coord": {
              "lon": "5.7357819",
              "lat": "45.1875602"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    },
    {
      "id": "stop_area:SNCF:87697466",
      "name": "Grenoble - Eugène Chavant (Grenoble)",
      "quality": 80,
      "stop_area": {
        "id": "stop_area:SNCF:87697466",
        "name": "Grenoble - Eugène Chavant",
        "codes": [
          {
            "type": "source",
            "value": "87697466"
          },
          {
            "type": "uic",
            "value": "87697466"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble - Eugène Chavant (Grenoble)",
        "coord": {
          "lon": "5.724751",
          "lat": "45.172844"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38185",
            "name": "Grenoble",
            "level": 8,
            "zip_code": "38000;38100",
            "label": "Grenoble (38000-38100)",
            "insee": "38185",
            "coord": {
              "lon": "5.7357819",
              "lat": "45.1875602"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    },
    {
      "id": "stop_area:SNCF:87742056",
      "name": "Grenoble Cité Internationale (Grenoble)",
      "quality": 80,
      "stop_area": {
        "id": "stop_area:SNCF:87742056",
        "name": "Grenoble Cité Internationale",
        "codes": [
          {
            "type": "source",
            "value": "87742056"
          },
          {
            "type": "uic",
            "value": "87742056"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble Cité Internationale (Grenoble)",
        "coord": {
          "lon": "5.710879",
          "lat": "45.195332"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38185",
            "name": "Grenoble",
            "level": 8,
            "zip_code": "38000;38100",
            "label": "Grenoble (38000-38100)",
            "insee": "38185",
            "coord": {
              "lon": "5.7357819",
              "lat": "45.1875602"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    },
    {
      "id": "stop_area:SNCF:87697474",
      "name": "Grenoble Victor Hugo (Grenoble)",
      "quality": 80,
      "stop_area": {
        "id": "stop_area:SNCF:87697474",
        "name": "Grenoble Victor Hugo",
        "codes": [
          {
            "type": "source",
            "value": "87697474"
          },
          {
            "type": "uic",
            "value": "87697474"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble Victor Hugo (Grenoble)",
        "coord": {
          "lon": "5.724939",
          "lat": "45.188405"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38185",
            "name": "Grenoble",
            "level": 8,
            "zip_code": "38000;38100",
            "label": "Grenoble (38000-38100)",
            "insee": "38185",
            "coord": {
              "lon": "5.7357819",
              "lat": "45.1875602"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    },
    {
      "id": "stop_area:SNCF:87335521",
      "name": "Grenoble - gare Routière (Grenoble)",
      "quality": 70,
      "stop_area": {
        "id": "stop_area:SNCF:87335521",
        "name": "Grenoble - gare Routière",
        "codes": [
          {
            "type": "source",
            "value": "87335521"
          },
          {
            "type": "uic",
            "value": "87335521"
          }
        ],
        "timezone": "Europe/Paris",
        "label": "Grenoble - gare Routière (Grenoble)",
        "coord": {
          "lon": "5.714366",
          "lat": "45.192693"
        },
        "links": [],
        "administrative_regions": [
          {
            "id": "admin:fr:38185",
            "name": "Grenoble",
            "level": 8,
            "zip_code": "38000;38100",
            "label": "Grenoble (38000-38100)",
            "insee": "38185",
            "coord": {
              "lon": "5.7357819",
              "lat": "45.1875602"
            }
          }
        ]
      },
      "embedded_type": "stop_area"
    }
  ],
  "context": {
    "current_datetime": "20250905T193810",
    "timezone": "Europe/Paris"
  },
  "links": [
    {
      "href": "https://api.sncf.com/v1/coverage/sncf/stop_areas/{stop_area.id}",
      "templated": true,
      "rel": "stop_areas",
      "type": "stop_area"
    }
  ]
}"#
    .to_string()
}

pub fn journeys() -> String {
    r#"{
  "journeys": [
    {
      "nb_transfers": 1,
      "departure_date_time": "20260103T062100",
      "arrival_date_time": "20260103T064900",
      "requested_date_time": "20260102T194657",
      "type": "best"
    },
    {
      "nb_transfers": 0,
      "departure_date_time": "20260103T075400",
      "arrival_date_time": "20260103T080300",
      "requested_date_time": "20260102T194657",
      "type": "fastest"
    }
  ]
}"#
    .to_string()
}

pub fn journeys_invalid_date() -> String {
    r#"{
  "journeys": [
    {
      "nb_transfers": 1,
      "departure_date_time": "I'm invalid",
      "arrival_date_time": "20260103T064900",
      "requested_date_time": "20260102T194657",
      "type": "best"
    },
    {
      "nb_transfers": 0,
      "departure_date_time": "20260103T075400",
      "arrival_date_time": "20260103T080300",
      "requested_date_time": "20260102T194657",
      "type": "fastest"
    }
  ]
}"#
    .to_string()
}
