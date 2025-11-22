Command I used for testing
```bash
curl -X POST http://localhost:6526/shop/recieve_order   -H "Content-Type: application/json"   -d '{
    "customer_info": {
      "order_id": "93616598-94e1-4b54-ae94-ccce8393d8bb",
      "email": "brock.tomlinson@ucalgary.ca",
      "phone": "2509466196",
      "name": "Brock",
      "sub_team": "Software"
    },
    "cart_items": [
      {
        "order_id": "93616598-94e1-4b54-ae94-ccce8393d8bb",
        "item_id": "HERO-2020 HOODIES",
        "size": "S",
        "quantity": 3,
        "price": 36.01
      },
      {
        "order_id": "93616598-94e1-4b54-ae94-ccce8393d8bb",
        "item_id": "HERO-2020 HOODIES",
        "size": "XL",
        "quantity": 10,
        "price": 36.01
      }
    ]
  }'
```