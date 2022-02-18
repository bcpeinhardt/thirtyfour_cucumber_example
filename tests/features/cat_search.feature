Feature: Kitty feature

    Scenario: Searching for cats on wikipedia
        Given browser successfully navigates to wikipedia
        When I search for cats
        Then browser navigates to https://en.wikipedia.org/wiki/Cat