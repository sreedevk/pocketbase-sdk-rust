migrate((db) => {
  const collection = new Collection({
    "id": "v69maop3x7zyvk4",
    "created": "2023-04-19 23:20:44.699Z",
    "updated": "2023-04-19 23:20:44.699Z",
    "name": "posts",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "v8ppnkrt",
        "name": "slug",
        "type": "text",
        "required": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "pattern": ""
        }
      },
      {
        "system": false,
        "id": "hcprqxtz",
        "name": "content",
        "type": "editor",
        "required": false,
        "unique": false,
        "options": {}
      },
      {
        "system": false,
        "id": "dmkivgcf",
        "name": "published_at",
        "type": "date",
        "required": false,
        "unique": false,
        "options": {
          "min": "",
          "max": ""
        }
      },
      {
        "system": false,
        "id": "8yla3is6",
        "name": "author_contact",
        "type": "email",
        "required": false,
        "unique": false,
        "options": {
          "exceptDomains": null,
          "onlyDomains": null
        }
      },
      {
        "system": false,
        "id": "l2v6rjvw",
        "name": "is_live",
        "type": "bool",
        "required": false,
        "unique": false,
        "options": {}
      }
    ],
    "indexes": [],
    "listRule": null,
    "viewRule": null,
    "createRule": null,
    "updateRule": null,
    "deleteRule": null,
    "options": {}
  });

  return Dao(db).saveCollection(collection);
}, (db) => {
  const dao = new Dao(db);
  const collection = dao.findCollectionByNameOrId("v69maop3x7zyvk4");

  return dao.deleteCollection(collection);
})
