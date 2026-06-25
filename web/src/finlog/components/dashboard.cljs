;; @file    <finlog/components/dashboard.cljs>
;; @author  <wakaranakattari@gmail.com>
;; @info    <dashboard page component with greeting, stats, and recent expenses>
;; @version <1.0>

;; @secstart->@secname <nsrq>
(ns finlog.components.dashboard
  (:require [reagent.core :as r]
            [re-frame.core :as rf]
            [clojure.string :as str]))
;; @secend->@secname <nsrq>

;; @secstart->@secname <greetingdata>
  ;; @funcinfo <returns greeting text and css class based on current hour>
(defn greeting-data []
  (let [hour (.getHours (js/Date.))]
    (cond
      (and (>= hour 6)  (< hour 12)) {:text "Good morning, "   :class "greeting-cyan"}
      (and (>= hour 12) (< hour 18)) {:text "Good afternoon, " :class "greeting-green"}
      (and (>= hour 18) (< hour 24)) {:text "Good evening, "   :class "greeting-yellow"}
      :else                           {:text "Good night, "     :class "greeting-blue"})))
;; @secend->@secname <greetingdata>

;; @secstart->@secname <formattime>
  ;; @funcinfo <returns current time as HH:MM:SS string>
(defn format-time []
  (let [d (js/Date.)
        pad #(-> % str (.padStart 2 "0"))]
    (str (pad (.getHours d)) ":" (pad (.getMinutes d)) ":" (pad (.getSeconds d)))))
;; @secend->@secname <formattime>

;; @secstart->@secname <formatdate>
  ;; @funcinfo <returns current date as DD.MM.YYYY string>
(defn format-date []
  (let [d (js/Date.)
        pad #(-> % str (.padStart 2 "0"))]
    (str (pad (.getDate d)) "." (pad (inc (.getMonth d))) "." (.getFullYear d))))
;; @secend->@secname <formatdate>

;; @secstart->@secname <clock>
  ;; @funcinfo <reagent component that updates displayed time every second>
(defn clock []
  (let [time (r/atom (format-time))]
    ;; @info <tick every 1000ms and reset atom>
    (js/setInterval #(reset! time (format-time)) 1000)
    (fn [] [:span @time])))
;; @secend->@secname <clock>

;; @secstart->@secname <recentexpenses>
  ;; @funcinfo <filters expenses list to only those within the last 7 days>
(defn recent-expenses [expenses]
  (let [now (js/Date.)
        week-ago (doto (js/Date.) (.setDate (- (.getDate now) 7)))]
    (filter (fn [{:keys [date]}]
              ;; @info <parse DD.MM.YYYY into js/Date for comparison>
              (let [[d m y] (str/split date #"\.")
                    exp-date (js/Date. y (dec (js/parseInt m)) (js/parseInt d))]
                (>= exp-date week-ago)))
            expenses)))
;; @secend->@secname <recentexpenses>

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
  ;; @funcinfo <renders list of recent expenses or empty state messages>
(defn expenses-list [expenses]
  (let [recent (recent-expenses expenses)]
    (cond
      ;; @info <no expenses at all>
      (empty? expenses)
      [:div.empty-state
       [:div.empty-state-title "No expenses"]
       [:div.empty-state-text "Add expenses in CLI version"]]

      ;; @info <expenses exist but none in the last 7 days>
      (empty? recent)
      [:div.empty-state
       [:div.empty-state-title "No recent expenses"]]

      ;; @info <render each recent expense as a card>
      :else
      [:div.expenses-list
       (for [exp recent]
         ^{:key (:name exp)} [expense-card exp])])))
;; @secend->@secname <expenseslist>

;; @secstart->@secname <statssummary>
  ;; @funcinfo <renders weekly total spending and record count from recent expenses>
(defn stats-summary [expenses]
  (let [recent (recent-expenses expenses)
        total  (reduce + 0 (map :amount recent))]
    [:section.stats-summary
     [:div.stat-card
      [:span "Weekly total spent:"]
      [:span (.toLocaleString total "ru-RU") " EUR"]]
     [:div.stat-card
      [:span "Weekly records:"]
      [:span (count recent)]]]))
;; @secend->@secname <statssummary>

;; @secstart->@secname <header>
  ;; @funcinfo <app header with title, navigation buttons and username display>
(defn header []
  (let [username (rf/subscribe [:username])]
    (fn []
      [:header.header
       ;; @secstart->@secname <headertitle> :: @secinfo <app title>
       [:div.title [:p "Finlog"]]
       ;; @secend->@secname <headertitle>

       ;; @secstart->@secname <headernav> :: @secinfo <page navigation buttons>
       [:nav.buttons-menu
        [:button {:on-click #(rf/dispatch [:set-page :dashboard])}   "Dashboard"]
        [:button {:on-click #(rf/dispatch [:set-page :expenses])}    "Expenses"]
        [:button {:on-click #(rf/dispatch [:set-page :statistics])}  "Statistics"]]
       ;; @secend->@secname <headernav>

       ;; @info <displays current username from re-frame store>
       [:div.session-id [:span @username]]])))
;; @secend->@secname <header>

;; @secstart->@secname <dashboard>
  ;; @funcinfo <main dashboard page with greeting, exchange rate, stats and recent expenses>
(defn dashboard []
  (fn []
    (let [{:keys [text class]} (greeting-data)
          username @(rf/subscribe [:username])
          exps     @(rf/subscribe [:expenses])]
      [:main.main-container
       ;; @secstart->@secname <greetingsection> :: @secinfo <time-based greeting with date and clock>
       [:section.greeting-section
        [:span {:class class} text]
        [:span username]
        [:span (format-date)]
        [:span [clock]]]
       ;; @secend->@secname <greetingsection>

       ;; @secstart->@secname <exchangerate> :: @secinfo <static exchange rate display>
       [:section.exchange-rate-section
        [:span "EUR to RUB exchange rate:"]
        [:span "1 EUR ≈ 95 RUB"]]
       ;; @secend->@secname <exchangerate>

       ;; @info <weekly statistics summary>
       [stats-summary exps]

       ;; @secstart->@secname <recentexpensessection> :: @secinfo <list of last 7 days expenses>
       [:section.recent-expenses
        [:span "Recent expenses"]
        [expenses-list exps]]
       ;; @secend->@secname <recentexpensessection>
       ])))
;; @secend->@secname <dashboard>

;; @secstart->@secname <app>
  ;; @funcinfo <root component combining header and current page>
(defn app []
  [:div
   [header]
   [dashboard]])
;; @secend->@secname <app>