migrate((db) => {
  const dao = new Dao(db);
  const collection = dao.findCollectionByNameOrId("lzvcjk2pa32n2t4");

  return dao.deleteCollection(collection);
}, (db) => {
  const collection = new Collection({
    "id": "lzvcjk2pa32n2t4",
    "created": "2023-04-20 00:02:35.675Z",
    "updated": "2023-04-20 00:02:35.675Z",
    "name": "admins",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "ntpomacb",
        "name": "name",
        "type": "text",
        "required": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "pattern": ""
        }
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
})
