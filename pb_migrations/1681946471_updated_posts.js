migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("v69maop3x7zyvk4")

  // add
  collection.schema.addField(new SchemaField({
    "system": false,
    "id": "isk0hkoj",
    "name": "user_id",
    "type": "relation",
    "required": false,
    "unique": false,
    "options": {
      "collectionId": "_pb_users_auth_",
      "cascadeDelete": false,
      "minSelect": null,
      "maxSelect": 1,
      "displayFields": []
    }
  }))

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("v69maop3x7zyvk4")

  // remove
  collection.schema.removeField("isk0hkoj")

  return dao.saveCollection(collection)
})
