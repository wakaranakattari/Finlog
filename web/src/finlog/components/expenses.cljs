;; @file    <finlog/components/expenses.cljs>
;; @author  <wakaranakattari@gmail.com>
;; @info    <expenses page component with full list and total statistics>
;; @version <1.0>

;; @secstart->@secname <nsrq>
(ns finlog.components.expenses
  (:require [re-frame.core :as rf]))
;; @secend->@secname <nsrq>

;; @secstart->@secname <expensecard>
  ;; @funcinfo <renders a single expense card with name, category, amount and date>
(defn expense-card [{:keys [name category amount date]}]
  [:div.expense-card
   ;; @info <expense name>
   [:div.expense-name name]
   ;; @info <expense category badge>
   [:div.expense-category category]
   ;; @info <formatted amount with locale>
   [:div.expense-amount (.toLocaleString amount "ru-RU") " EUR"]
   ;; @info <expense date>
   [:div.expense-meta [:span date]]])
;; @secend->@secname <expensecard>

;; @secstart->@secname <expenseslist>
  ;; @funcinfo <renders all expenses from store or empty state if none exist>
(defn expenses-list []
  (let [expenses (rf/subscribe [:expenses])]
    (fn []
      (if (empty? @expenses)
        ;; @info <empty state when no expenses>
        [:div.empty-state
         [:div.empty-state-title "No expenses"]
         [:div.empty-state-text "Add expenses in CLI version"]]
        ;; @info <render each expense as a card>
        [:div.expenses-list
         (for [exp @expenses]
           ^{:key (:name exp)} [expense-card exp])]))))
;; @secend->@secname <expenseslist>

;; @secstart->@secname <statssummary>
  ;; @funcinfo <renders total spending and record count for all time>
(defn stats-summary []
  (let [expenses (rf/subscribe [:expenses])]
    (fn []
      (let [total (reduce + 0 (map :amount @expenses))]
        [:section.stats-summary
         [:div.stat-card
          [:span "Total spent for all time:"]
          [:span (.toLocaleString total "ru-RU") " EUR"]]
         [:div.stat-card
          [:span "Total records for all time:"]
          [:span (count @expenses)]]]))))
;; @secend->@secname <statssummary>

;; @secstart->@secname <expensespage>
  ;; @funcinfo <expenses page root component with stats and full expenses list>
(defn expenses-page []
  [:main.main-container.expenses-page
   ;; @info <all time statistics summary>
   [stats-summary]
   ;; @secstart->@secname <allexpenses> :: @secinfo <full list of all expenses>
   [:section.all-expenses
    [:span "All expenses"]
    [expenses-list]]
   ;; @secend->@secname <allexpenses>
   ])
;; @secend->@secname <expensespage>