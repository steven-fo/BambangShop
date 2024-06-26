# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [X] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [X] Commit: `Create Subscriber model struct.`
    -   [X] Commit: `Create Notification model struct.`
    -   [X] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [X] Commit: `Implement add function in Subscriber repository.`
    -   [X] Commit: `Implement list_all function in Subscriber repository.`
    -   [X] Commit: `Implement delete function in Subscriber repository.`
    -   [X] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [X] Commit: `Create Notification service struct skeleton.`
    -   [X] Commit: `Implement subscribe function in Notification service.`
    -   [X] Commit: `Implement subscribe function in Notification controller.`
    -   [X] Commit: `Implement unsubscribe function in Notification service.`
    -   [X] Commit: `Implement unsubscribe function in Notification controller.`
    -   [X] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [X] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [X] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [X] Commit: `Implement publish function in Program service and Program controller.`
    -   [X] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [X] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. Single model struct sudah cukup untuk kasus bambangshop kali ini. Interface diperlukan kalau ada beberapa tipe dengan behaviour yang berbeda-beda pada subscriber nya. Namun, pada modul ini, subscriber hanya memiliki 1 tipe atau semua subcriber sama, jadi tidak perlu menggunakan interface.
2. Menurut saya, penggunaan dashmap lebih cocok untuk kasus ini. DashMap menggunakan key input untuk mendapatkan value berdasarkan key tersebut. Jadi, pencarian data atau value akan lebih mudah karena kita hanya perlu meng-input id atau url saja.
3. Pada kasus ini, kedua singleton dan dashmap sama-sama diperlukan karena keduanya saling melengkapi. Singleton memastikan bahwa satu instance hanya akan ada di satu interface. Selain itu, dashmap akan memastikan bahwa data subscribers yang ada tetap aman dipakai oleh thread-thread yang berbeda.

#### Reflection Publisher-2

1. Menurut saya, service dan repository harus dipisah dari model agar lebih mudah dibaca dan diubah kalau ingin ada perubahan. Jika ketiga fungisionalitas tersebut digabung menjadi 1 file, tentunya file tersebut akan terlalu panjang dan susah untuk dibaca. Kalau ketiga fungsionalitas tersebut dipisah menjadi file-file nya masing-masing, setiap file akan lebih mudah dibaca dan mudha untuk diubah.
2. Jika kita hanya menggunakan model saja (service dan repository digabung menjadi 1 file), maka file tersebut akan susah dibaca. Ditambah, kali ini kita memiliki 3 model, yaitu program, subscriber, dan notification, tentunya file tersebut akan sangat panjang dan susah untuk dibaca. Kesulitan untuk dibaca juga mempengaruhi tingkat kesulitan mengubah atau mengutak-atik kodenya saat ingin ada perubahan.
3. Postman berfungsi untuk mengetes API dari suatu projek. Menurut saya, Postman sangat berguna dalam mengetes API karena fitur-fitur yang disediakan, seperti membuat request dengan berbagai macam token ataupun body request, bisa kustomisasi environment suatu collection, dan terdapat fitur monitoring juga.

#### Reflection Publisher-3

1. Pada kasus ini, kita menggunakan variasi <strong>push</strong> model dari observer pattern, dimana publisher melakukan push data atau update kepada subscriber.
2. Keuntungan dalam menggunakan variasi <strong>pull</strong> model adalah lebih efisien karena update atau perubahan hanya diterima ketika dibutuhkan saja. Kekurangan dari variasi ini adalah keterlambatan pembaruan dimana suatu informasi baru hanya dapat diterima saat subscriber meminta informasi tersebut.
3. Multi-threading pada projek ini digunakan agar dapat mengirimkan notifikasi kepada semua subscriber secara asinkronus. Jika multi-threading tidak digunakan, maka notifikasi yang dikirimkan ke subscriber akan semakin lama karena dilakukan secara sinkronus.