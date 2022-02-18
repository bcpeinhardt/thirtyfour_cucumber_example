Feature: Doggo feature

    Scenario: Searching for dogs on wikipedia
        Given browser successfully navigates to wikipedia
        When I search for dogs
        Then browser navigates to https://en.wikipedia.org/wiki/Dog

