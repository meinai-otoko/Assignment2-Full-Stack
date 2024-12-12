<template>
  <TopNav :cartItemCount="cartItemCount"/>
  <router-view
    :products="products"
    :cartItems="cartItems"
    @addToCart="addToCart"
    @removeFromCart="removeFromCart"
    @submitOrder="submitOrder"
  ></router-view>
</template>

<script>
import TopNav from './components/TopNav.vue'

export default {
  name: 'App',
  components: {
    TopNav
  },
  data() {
    return {
      cartItems: [],
      products: [],
    }
  },
  computed: {
    cartItemCount() {
      return this.cartItems.reduce((total, item) => {
        return total + item.quantity
      }, 0)
    }
  },
  mounted() {
    this.getProducts()
  },
  methods: {
    getProducts() {
      fetch('/products')
        .then(response => response.json())
        .then(products => {
          console.log('success getting proxy products')
          this.products = products
        })
        .catch(error => {
          console.log(error)
          alert('Error occurred while fetching products')
        })
    },
    addToCart({ productId, quantity }) {
      const existingCartItem = this.cartItems.find(
        item => item.product.id == productId
      )
      if (existingCartItem) {
        existingCartItem.quantity += quantity
      } else {
        const product = this.products.find(product => product.id == productId)
        this.cartItems.push({ product, quantity })
      }
    },
    removeFromCart(index) {
      this.cartItems.splice(index, 1)
    },
    submitOrder() {
      const order = {
        customerId: Math.floor(Math.random() * 10000000000).toString(),
        items: this.cartItems.map(item => {
          return {
            productId: item.product.id,
            quantity: item.quantity,
            price: item.product.price
          }
        })
      }

      console.log(JSON.stringify(order));

      fetch(`/order`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(order)
      })
        .then(response => {
          console.log(response)
          if (!response.ok) {
            alert('Error occurred while submitting order')
          } else {
            this.cartItems = []
            alert('Order submitted successfully')
          }
        })
        .catch(error => {
          console.log(error)
          alert('Error occurred while submitting order')
        })
    }
  },
}
</script>

<style>
body {
  background-color: #F0F2F5;
  margin: 0;
  padding: 0;
}

#app {
  font-family: 'Human BBY Digital', Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #0C121C;
  margin-top: 60px;
}

footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: #0046BE;
  color: #fff;
  padding: 1rem;
  margin: 0;
}

nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #0046BE;
  padding: 0 20px;
}

ul {
  display: flex;
  list-style: none;
  margin: 0;
  padding: 0;
}

li {
  margin: 0 1rem;
}

a {
  color: #fff;
  text-decoration: none;
}

button {
  padding: 10px 20px;
  background-color: #0046BE;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  height: 44px;
  font-weight: bold;
  font-size: 14px;
  transition: background-color 0.2s;
}

button:hover {
  background-color: #001E73;
}

.product-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
  padding: 24px;
  max-width: 1280px;
  margin: 0 auto;
}

.product-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  margin: 0;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  background-color: white;
  transition: transform 0.2s;
}

.product-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.product-card img {
  max-width: 100%;
  margin-bottom: 16px;
}

.product-card a {
  text-decoration: none;
  color: #0C121C;
}

.product-card h2 {
  font-weight: bold;
  margin-bottom: 8px;
  font-size: 1.2rem;
}

.product-card p {
  margin-bottom: 16px;
  color: #55555A;
}

.product-controls {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-top: 16px;
}

.product-controls p {
  margin-right: 20px;
}

.product-price {
  color: #0046BE;
  font-weight: bold;
  font-size: 1.75rem;
}

.quantity-input {
  width: 60px;
  height: 36px;
  border: 1px solid #E0E6EA;
  border-radius: 4px;
  padding: 8px;
  margin-right: 12px;
  font-size: 14px;
}

.shopping-cart {
  display: flex;
  flex-direction: column;
  align-items: center;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  padding: 32px;
  margin: 32px auto;
  max-width: 1280px;
}

.shopping-cart h2 {
  font-size: 28px;
  margin-bottom: 24px;
  color: #0C121C;
}

.shopping-cart-table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0 8px;
}

.shopping-cart-table th {
  background-color: #F0F2F5;
  padding: 16px;
  text-align: left;
  font-weight: bold;
  color: #0C121C;
}

.shopping-cart-table td {
  padding: 16px;
  border-bottom: 1px solid #E0E6EA;
}

.checkout-button {
  margin-top: 24px;
  padding: 12px 24px;
  background-color: #FFE000;
  color: #0C121C;
  font-weight: bold;
  font-size: 16px;
  transition: background-color 0.2s;
  height: 48px;
}

.checkout-button:hover {
  background-color: #FFC700;
}
</style>