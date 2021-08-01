**Todo-list Backend API**
-------------------------

**Description**
---------------
This is the backend API for a todo-list application where users can create an account, and store lists accessible from any device (through a frontend implementation). Additionally, lists can be shared with other users either by username or email, and then they will have the ability to edit those lists, although only list "owners" can delete lists entirely. In order to even start this project, I had to learn about Rocket, the framework I decided to use. I had many problems in my first few days working on this project, mostly down to my inexperience and the sparsity of documentation for issues I came across (although the Rocket documentation is pretty great). In the end, I elarned about API's, databases, asynchronous programming, and HTTP requests/responses throughout this project.


**Usage**
--------
In order to run this backend, you will need to create a secret.key file in the src/ directory containing your key used to encrypt all JWT tokens.

**Future Expansion/Things Left to Do**
--------------------------------------
Here is my to-do list (ironic enough) of things I still need to do/implement, or features I'd like to add:
1. A POST route to share a list with another user, although the basic infrastructure is already in place.
2. Finalize my route links and document them below with sample JSON request data.

**Routes**
----------
*to-do*

**Credits**
-----------
I took a lot of inspiration and examples from these projects for figuring out how to structure
and implement the User authentication, specifically with JWT tokens:
- https://github.com/SakaDream/rocket-rest-api-with-jwt
- https://github.com/TatriX/realworld-rust-rocket
