<template>
  <TopNav />
  <router-view
    :orders="orders"
    :products="products"
    @fetchOrders="fetchOrders"
    @completeOrder="completeOrder"
    @addProductsToList="addProductsToList"
    @updateProductInList="updateProductInList"
    @getProduct="getProduct"
    @getProducts="getProducts"
  ></router-view>
</template>

<script>
import TopNav from './components/TopNav.vue';

const productServiceUrl = "/products/";
const singleProductServiceUrl = "/product/";
const makelineServiceUrl = "/makeline/";

export default {
  name: 'App',
  components: {
    TopNav
  },
  data() {
    return {
      orders: [],
      products: [],
      product: {}
    }
  },
  mounted() {
    this.getProducts();
  },
  methods: {
    async addProductsToList(newProduct) {
      this.products.push(newProduct);
    },
    async updateProductInList(updatedProduct) {
      const index = this.products.findIndex(product => product.id === updatedProduct.id);
      this.products[index] = updatedProduct;
    },
    async getProduct(id) {
      fetch(`${singleProductServiceUrl}${id}`)
        .then(response => response.json())
        .then(product => {
          this.product.id = product.id
          this.product.name = product.name
          this.product.image = product.image
          this.product.description = product.description
          this.product.price = product.price
        })
        .catch(error => {
          console.log(error)
          alert('Error occurred while fetching product')
        })
    },
    async getProducts() {
      fetch(`${productServiceUrl}`)
        .then(response => response.json())
        .then(products => {
          this.products = products
        })
        .catch(error => {
          console.log(error)
          alert('Error occurred while fetching products')
        })
    },
    async fetchOrders() {
      await fetch(`${makelineServiceUrl}order/fetch`)
        .then(response => response.json())
        .then(data => {
          console.log(data)
          if (data) {
            this.orders = data;
          } else {
            console.log('No orders from server');
          }
        })
        .catch(error => console.error(error));
    },
    async completeOrder(orderId) {      
      let order = this.orders.find(order => order.orderId === orderId);
      order.status = 1;

      let orderObject = JSON.stringify(order)
      console.log(orderObject);

      await fetch(`${makelineServiceUrl}order`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json'
        },
        body: orderObject
      })
        .then(response => {
          if (!response.ok) {
            alert('Error occurred while processing order')
          } else {
            alert('Order successfully processed')
            this.orders = this.orders.filter(order => order.orderId !== orderId);
            this.$router.go(-1);
          }
        })
        .catch(error => {
          console.log(error)
          alert('Error occurred while processing order')
        })
    }
  },
}
</script>

<style>
#app {
  font-family: "Human BBY Digital", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #0046BE;
  margin-top: 80px;
  padding: 2rem;
  background-color: #f0f2f4;
}

table {
  width: 100%;
  border-collapse: collapse;
  border-spacing: 0;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

th, td {
  padding: 12px 16px;
  text-align: left;
  border-bottom: 1px solid #e5e5e5;
}

th {
  background-color: #f8f9fa;
  color: #0046BE;
  font-weight: 600;
}

button {
  padding: 10px 20px;
  background-color: #0046BE;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  height: 42px;
  transition: background-color 0.2s;
}

button:hover {
  background-color: #003089;
}

.action-button {
  float: right;
}

.product-detail {
  text-align: left;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  gap: 2rem;
  margin: 2rem auto;
  background-color: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.product-form {
  display: flex;
  flex-direction: column;
  align-items: left;
  justify-content: center;
  margin: 2rem auto;
  width: 50%;
  background-color: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.form-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.ai-button {
  margin-left: 10px;
  padding: 10px 20px;
  border-radius: 4px;
  border: none;
  background-color: #FFE000;
  color: #0046BE;
  cursor: pointer;
  font-weight: 600;
}

.ai-button:hover {
  background-color: #FFD100;
}

textarea {
  width: 100%;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #d2d2d2;
  resize: vertical;
}

label {
  text-align: right;
  margin-right: 1rem;
  width: 120px;
  font-weight: 600;
  color: #0046BE;
}

input {
  width: 100%;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #d2d2d2;
  transition: border-color 0.2s;
}

input:focus, textarea:focus {
  outline: none;
  border-color: #0046BE;
}

.order-detail {
  text-align: left;
  background-color: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}
</style>