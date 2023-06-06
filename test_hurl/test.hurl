GET {{some}}/api/cool?par=test
Accept: application/json

HTTP 200
[Asserts]
jsonpath "$.c" exists
jsonpath "$.r" == "test"
