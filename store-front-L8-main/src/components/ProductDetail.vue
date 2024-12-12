<template>
  <div class="product-detail-container">
    <div class="product-detail" v-if="productExists">
      <div class="product-image">
        <img :src="product.image" :alt="product.name" />
      </div>
      <div class="product-info">
        <h1>{{ product.name }}</h1>
        <div class="model-number">Model: {{ product.id }}</div>
        <div class="product-price">${{ product.price }}</div>
        <p class="description">{{ product.description }}</p>
        <div class="product-controls">
          <div class="quantity-wrapper">
            <input 
              type="number" 
              v-model="quantity" 
              min="1" 
              class="quantity-input"
              aria-label="Quantity" 
            />
          </div>
          <button @click="addToCart" class="add-to-cart-btn">Add to Cart</button>
        </div>
      </div>
    </div>
    <div class="product-detail not-found" v-else>
      <img src="../assets/404.jpg" alt="Product not found" />
      <h2>Sorry! We couldn't find that product</h2>
      <router-link to="/" class="return-link">Return to Products</router-link>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ProductDetail',
  props: ['products'],
  data() {
    return {
      quantity: 1
    }
  },
  computed: {
    product() {
      return this.products.find(product => product.id == this.$route.params.id);
    },
    productExists() {
      return !!this.product;
    }
  },
  methods: {
    addToCart() {
      this.$emit('addToCart', {
        productId: this.product.id,
        quantity: this.quantity
      })
    }
  }
}
</script>

<style scoped>
.product-detail-container {
  max-width: 1280px;
  margin: 80px auto 32px;
  padding: 0 24px;
}

.product-detail {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 48px;
  background: white;
  border-radius: 8px;
  padding: 32px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.product-image img {
  width: 100%;
  height: auto;
  object-fit: contain;
}

.product-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

h1 {
  font-size: 32px;
  color: #0C121C;
  margin: 0;
}

.model-number {
  color: #55555A;
  font-size: 14px;
}

.product-price {
  color: #0046BE;
  font-size: 32px;
  font-weight: 700;
  margin: 16px 0;
}

.description {
  color: #55555A;
  font-size: 16px;
  line-height: 1.6;
  margin: 0;
}

.product-controls {
  display: flex;
  gap: 16px;
  margin-top: 24px;
}

.quantity-input {
  width: 80px;
  height: 48px;
  border: 1px solid #E0E6EA;
  border-radius: 4px;
  padding: 8px;
  font-size: 16px;
  text-align: center;
}

.add-to-cart-btn {
  flex-grow: 1;
  height: 48px;
  background-color: #FFE000;
  color: #0C121C;
  border: none;
  border-radius: 4px;
  font-weight: 700;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.add-to-cart-btn:hover {
  background-color: #FFC700;
}

.not-found {
  text-align: center;
  padding: 48px;
}

.not-found img {
  max-width: 300px;
  margin-bottom: 24px;
}

.not-found h2 {
  color: #0C121C;
  margin-bottom: 16px;
}

.return-link {
  color: #0046BE;
  text-decoration: none;
  font-weight: 500;
}

.return-link:hover {
  text-decoration: underline;
}

@media (max-width: 768px) {
  .product-detail {
    grid-template-columns: 1fr;
    gap: 32px;
    padding: 24px;
  }
}
</style>