export const closeModal = (cardElement) => {
  cardElement.classList.remove('card-visible');
  cardElement.classList.add('card-hidden');
}

export const openModal = (cardElement) => {
  cardElement.classList.remove('card-hidden');
  cardElement.classList.add('card-visible');
}

export const initModal = () => {
  const signupBtn = document.getElementById('signup');
  const signupId = document.getElementById('signup-modal');

  const loginBtn = document.getElementById('login');
  const loginId = document.getElementById('login-modal');

  signupBtn.addEventListener('click', () => {
    console.log("Button clicked");
    openModal(signupId);
    closeModal(loginId);
  });

  loginBtn.addEventListener('click', () => {
    openModal(loginId);
    closeModal(signupId)
  });

  signupId.addEventListener('click', (event) => {
    if (event.target === signupId) {
      closeModal(signupId);
    }
  });

  loginId.addEventListener('click', (event) => {
    if (event.target === loginId) {
      closeModal(loginId);
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(loginId);
      closeModal(signupId);
    }
  })
}
