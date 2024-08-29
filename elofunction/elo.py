# Fighter class with Elo rating
class Fighter:
    def __init__(self, name, rating=1500):
        self.name = name
        self.rating = rating

    # Method to update the rating after a fight
    def update_rating(self, opponent, result):
        K = 50  # This is the default value for K in Elo system, can be adjusted

        if result == "win":
            expected_score = 1 / (1 + 10 ** ((opponent.rating - self.rating) / 400))
            self.rating += K * (1 - expected_score)

        elif result == "loss":
            expected_score = 1 / (1 + 10 ** ((self.rating - opponent.rating) / 400))
            self.rating += K * (0 - expected_score)


# Create some fighters
fighter1 = Fighter("Conor McGregor")
fighter2 = Fighter("Khabib Nurmagomedov")

#print initial ratings and name of both fighters
print("Initial Ratings before match")
print(f"Name: {fighter1.name}, Rating: {fighter1.rating}")
print(f"Name: {fighter2.name}, Rating: {fighter2.rating}")

# Simulate a fight and update ratings
fighter1.update_rating(fighter2, "loss")
fighter2.update_rating(fighter1, "win")
print(f"Winner: {fighter2.name}")

# Print ratings and name of both fighters
print("Ratings after match")
print(f"Name: {fighter1.name}, Rating: {fighter1.rating}")
print(f"Name: {fighter2.name}, Rating: {fighter2.rating}")
