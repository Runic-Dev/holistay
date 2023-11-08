export enum Bed {
  Single,
  Double,
  Queen,
  King
}

export const Properties = {
  "data": [
    {
      "id": "001",
      "name": "Black Prince",
      "imageUrl": "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1b/ed/95/07/limak-eurasia-luxury.jpg?w=1100&h=-1&s=1",
      "address": {
        "street": "123 Dark Alley",
        "city": "London",
        "state": "N/A",
        "postalCode": "SW1A 1AA",
        "country": "UK"
      },
      "contact": {
        "name": "John Doe",
        "phone": "123-456-7890",
        "email": "john.doe@example.com"
      },
      "roomGroups": [
        {
          "id": "110",
          "name": "Gîte",
          "imageUrl": "https://www.vrbo.com/en-gb/holiday-homes/wp-content/uploads/1UBzswqLL6qvYUBUnTpKJI/02b4a9d307eb8fbcbd486b45dd95d267/2.dordogne-p906974a-3.jpg",
          "description": "A cozy, rustic experience for the adventurous.",
          "rooms": [
            {
              "id": "111",
              "number": 101,
              "imageUrl": "https://imgs.search.brave.com/tvuYS1ojA168YBGw_syCk7K-35VJqfrIibBdFS7gZc8/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5pc3RvY2twaG90/by5jb20vaWQvODQz/ODIzNjU2L3Bob3Rv/L2hvdGVsLXJvb20u/anBnP3M9NjEyeDYx/MiZ3PTAmaz0yMCZj/PTgtWk5BNTJlNUds/UHV1UVBYcVpSZ3NU/TzlXUlp3WmdGdERv/dHlDNkNHSFk9",
              "name": "The Dungeon",
              "beds": [Bed.Double, Bed.Single]
            },
            {
              "id": "112",
              "number": 102,
              "imageUrl": "https://imgs.search.brave.com/tvuYS1ojA168YBGw_syCk7K-35VJqfrIibBdFS7gZc8/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5pc3RvY2twaG90/by5jb20vaWQvODQz/ODIzNjU2L3Bob3Rv/L2hvdGVsLXJvb20u/anBnP3M9NjEyeDYx/MiZ3PTAmaz0yMCZj/PTgtWk5BNTJlNUds/UHV1UVBYcVpSZ3NU/TzlXUlp3WmdGdERv/dHlDNkNHSFk9",
              "name": "The Tower",
              "beds": [Bed.Queen]
            }
          ]
        }
      ]
    },
    {
      "id": "002",
      "name": "Another Hotel",
      "imageUrl": "https://www.peninsula.com/en/-/media/01-hong-kong-property/exterior-2_5000x2811.jpg?mw=905&hash=934337A3F240DDF77532E22A9E1EA451",
      "address": {
        "street": "456 Fancy St",
        "city": "New York",
        "state": "NY",
        "postalCode": "10001",
        "country": "USA"
      },
      "contact": {
        "name": "Jane Doe",
        "phone": "987-654-3210",
        "email": "jane.doe@example.com"
      },
      "roomGroups": [
        {
          "id": "210",
          "name": "Penthouse",
          "imageUrl": "https://www.paranych.com/thumbs/uploads/benefits-penthouse-living-main-image.png",
          "description": "Luxury living with a view that'll knock your socks off.",
          "rooms": [
            {
              "id": "211",
              "number": 501,
              "name": "Sky High",
              "beds": [Bed.King]
            },
            {
              "id": "212",
              "number": 502,
              "imageUrl": "https://imgs.search.brave.com/tvuYS1ojA168YBGw_syCk7K-35VJqfrIibBdFS7gZc8/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5pc3RvY2twaG90/by5jb20vaWQvODQz/ODIzNjU2L3Bob3Rv/L2hvdGVsLXJvb20u/anBnP3M9NjEyeDYx/MiZ3PTAmaz0yMCZj/PTgtWk5BNTJlNUds/UHV1UVBYcVpSZ3NU/TzlXUlp3WmdGdERv/dHlDNkNHSFk9",
              "name": "Cloud Nine",
              "beds": [Bed.Queen, Bed.Single]
            }
          ]
        }
      ]
    }
  ]
}


