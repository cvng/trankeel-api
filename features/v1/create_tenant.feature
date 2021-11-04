Feature: CreateTenant
  Scenario: Ok
    Given an Account
    When I create a Tenant
    Then I should have a Tenant
    Then I should have a list of Warrant
    Then I should have a Discussion
