// All of the messages that can be returned for each API call.
pub const MESSAGE_OK: &str = "ok";
// pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "can not insert data";
// pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "can not update data";
// pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "signed up successfully";
pub const MESSAGE_SIGNUP_FAILED: &str = "error when signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "logged in successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "wrong username or password, please try again";
pub const MESSAGE_INVALID_TOKEN: &str = "invalid token, please login again";
pub const MESSAGE_GET_USER_SUCCESS: &str = "found user successfully";
pub const MESSAGE_GET_USER_FAILED: &str = "wrong username, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "could not identify the given user, please try again";

pub const MESSAGE_CREATE_LIST_SUCCESS: &str = "created list successfully";
pub const MESSAGE_CREATE_LIST_FAILED: &str = "error when creating list, please try again";

pub const MESSAGE_CREATE_LIST_ASSOCIATE_FAIL: &str = "error when associating the list with its owner, please try again";

pub const MESSAGE_DELETE_LIST_FAILED: &str = "error when deleting list, please try again";
pub const MESSAGE_DELETE_NOT_OWNER: &str = "error deleting list, user is not an owner";
pub const MESSAGE_DELETE_LIST_SUCCESS: &str = "deleted list successfully";

pub const MESSAGE_CREATE_ITEM_SUCCESS: &str = "created item successfully";
pub const MESSAGE_CREATE_ITEM_FAILED: &str = "error when creating item, please try again";

pub const MESSAGE_NO_ACCESS: &str = "you do not have access to the specified resource";

pub const MESSAGE_DELETE_ITEM_SUCCESS: &str = "deleted item successfully";
pub const MESSAGE_DELETE_ITEM_FAILED: &str = "error when deleting item, please try again";

pub const MESSAGE_GET_LIST_SUCCESS: &str = "got list successfully";
pub const MESSAGE_GET_LIST_FAILED: &str = "error when getting list, please try again";

pub const MESSAGE_UPDATE_LIST_SUCCESS: &str = "updated list successfully";
pub const MESSAGE_UPDATE_LIST_FAILED: &str = "error when updating list, please try again";

pub const MESSAGE_UPDATE_ITEM_SUCCESS: &str = "updated item successfully";
pub const MESSAGE_UPDATE_ITEM_FAILED: &str = "error when updating item, please try again";

pub const MESSAGE_ITEM_NOT_OWNED_BY_LIST: &str = "error, item not owned by the specified list";
