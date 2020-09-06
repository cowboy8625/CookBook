import random
import os

RANKS = "2 3 4 5 6 7 8 9 10 J Q K A".split()
SUITS = "♣ ♢ ♡ ♠".split()

def clear():
    os.system("cls" if os.name == "nt" else "clear")


class Card:
    def __init__(self, rank, suit):
        self.rank = rank
        self.suit = suit

    def __repr__(self):
        return f"{self.suit} {self.rank}"


class Deck:
    def __init__(self):
        self._deck = []
        self.create()

    def create(self):
        for suit in SUITS:
            for rank in RANKS:
                self._deck.append(Card(rank, suit))

    def shuffle(self):
        random.shuffle(self._deck)

    def get_card(self, n=1):
        result = []
        for card in enumerate(self._deck, start=1):
            result.append(card)    
            if idx == n:
                return result


    def show(self):
        new = []
        new.append(self._deck[0:13])
        new.append(self._deck[13 : 13 * 2])
        new.append(self._deck[13 * 2 : 13 * 3])
        new.append(self._deck[13 * 3 : :])
        for line in new:
            print(*line)


class Account:
    account = 0

    def __init__(self, amount):
        self.deposit(amount)

    def deposit(self, amount):
        self.account += amount

    def withdraw(self, amount):
        if amount < self.account:
            self.account -= amount
            return amount
        else:
            amount = self.account
            self.account -= self.account
            return amount

    def balance(self):
        return self.account


class Player:
    def __init__(self, name, amount, hand):
        self.name = name
        self.chips = Account(amount)
        self.hand = hand


class Dealer:
    def __init__(self, table_count, card_count):
        self.table_count = table_count
        self.card_count = card_count
        self.deck = Deck()
        self.table = []
        self.deal()

    def deal(self):
        self.deck.shuffle()
        for i in range(self.table_count):
            self.table.append(Player(f"{i}", 100, self.deck.get_card(n=self.card_count)))


class Game:
    def __init__(self):
        self.state = self.start

    def start(self):
        self.players = input("How many players are there?:> ") 
        self.state = self.player_names

    def player_names(self):
        for i in range(self.players):
            pass



game = Game()
while True:
    game.state()




