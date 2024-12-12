<template>
  <div class="product-card">
    <img :src="product.image" alt="Product Image">
    <router-link :to="`/product/${product.id}`">
      <h2>{{ product.name }}</h2>
    </router-link>
    <p>{{ product.description }}</p>
    <div class="product-details">
      <div class="product-price">
        <p class="price">${{ product.price }}</p>
      </div>
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
</template>

<script>
export default {
  name: 'ProductCard',
  props: ['product'],
  data() {
    return {
      quantity: 1
    }
  },
  methods: {
    incrementQuantity() {
      this.quantity++
    },
    decrementQuantity() {
      if (this.quantity > 1) {
        this.quantity--
      }
    },
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
.product-card {
  background: white;
  border-radius: 8px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.product-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.product-card img {
  width: 100%;
  height: auto;
  object-fit: contain;
  margin-bottom: 16px;
}

.product-card h2 {
  color: #0046BE;
  font-size: 18px;
  font-weight: 700;
  margin: 0;
  text-align: left;
}

.product-card p {
  color: #55555A;
  font-size: 14px;
  line-height: 1.5;
  margin: 0;
  text-align: left;
}

.product-details {
  margin-top: auto;
}

.product-price {
  margin-bottom: 16px;
}

.price {
  color: #0046BE;
  font-size: 24px;
  font-weight: 700;
  margin: 0;
}

.product-controls {
  display: flex;
  gap: 12px;
  align-items: center;
}

.quantity-wrapper {
  flex-shrink: 0;
}

.quantity-input {
  width: 64px;
  height: 40px;
  border: 1px solid #E0E6EA;
  border-radius: 4px;
  padding: 8px;
  font-size: 14px;
  text-align: center;
}

.add-to-cart-btn {
  flex-grow: 1;
  height: 40px;
  background-color: #FFE000;
  color: #0C121C;
  border: none;
  border-radius: 4px;
  font-weight: 700;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.add-to-cart-btn:hover {
  background-color: #FFC700;
}
</style>