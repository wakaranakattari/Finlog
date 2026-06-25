;; @file    <finlog/components/statistics.cljs>
;; @author  <wakaranakattari@gmail.com>
;; @info    <statistics page with chart.js visualizations for expenses>
;; @version <1.0>

;; @secstart->@secname <nsrq>
(ns finlog.components.statistics
  (:require [re-frame.core :as rf]
            [reagent.core :as r]
            [clojure.string :as str]))
;; @secend->@secname <nsrq>

;; @secstart->@secname <chartcolors>
  ;; @info <palette used across all charts>
(def chart-colors ["#ffdab3" "#c8aaaa" "#9f8383" "#574964" "#8fbc8f" "#e8c97a"])
;; @secend->@secname <chartcolors>

;; @secstart->@secname <chartdefaults>
  ;; @info <shared chart.js options applied to all charts>
(def chart-defaults
  {:responsive true
   :maintainAspectRatio false
   :plugins {:legend {:display false}
             :tooltip {:backgroundColor "#2a1f33"
                       :titleColor "#f5ede4"
                       :bodyColor "#c4b5a8"
                       :borderColor "#3d3047"
                       :borderWidth 2
                       :cornerRadius 4}}
   :scales {:y {:beginAtZero true
                :grid {:color "#3d3047"
                       :drawBorder false}
                :ticks {:color "#c4b5a8"
                        :font {:family "Pixelify Sans"
                               :size 14}}}
            :x {:grid {:display false}
                :ticks {:color "#c4b5a8"
                        :font {:family "Pixelify Sans"
                               :size 14}}}}})
;; @secend->@secname <chartdefaults>

;; @secstart->@secname <bycategory>
  ;; @funcinfo <aggregates expenses into a map of category -> total amount>
(defn by-category [expenses]
  (reduce (fn [acc {:keys [category amount]}]
            (update acc category (fnil + 0) amount))
          {} expenses))
;; @secend->@secname <bycategory>

;; @secstart->@secname <bydate>
  ;; @funcinfo <aggregates expenses into a map of date -> total amount>
(defn by-date [expenses]
  (reduce (fn [acc {:keys [date amount]}]
            (update acc date (fnil + 0) amount))
          {} expenses))
;; @secend->@secname <bydate>

;; @secstart->@secname <bymonth>
  ;; @funcinfo <aggregates expenses into a map of YYYY-MM -> total amount>
(defn by-month [expenses]
  (reduce (fn [acc {:keys [date amount]}]
            (let [parts (str/split date #"\.")
                  month (str (nth parts 2) "-" (nth parts 1))]
              (update acc month (fnil + 0) amount)))
          {} expenses))
;; @secend->@secname <bymonth>

;; @secstart->@secname <makechart>
  ;; @funcinfo <creates a chart.js instance on the given canvas id with type, data and options>
(defn make-chart [canvas-id type data options]
  (when-let [ctx (some-> (.getElementById js/document canvas-id)
                         (.getContext "2d"))]
    (js/Chart. ctx (clj->js {:type type :data data :options options}))))
;; @secend->@secname <makechart>

;; @secstart->@secname <categorychart>
  ;; @funcinfo <bar chart of total spending per category, renders empty state if no expenses>
(defn category-chart [expenses]
  (r/create-class
   {:component-did-mount
    (fn []
      (when (seq expenses)
        (let [cats   (by-category expenses)
              labels (clj->js (keys cats))
              data   (clj->js (vals cats))]
          (make-chart "category-chart" "bar"
                      {:labels labels
                       :datasets [{:data data
                                   :backgroundColor "rgba(255,218,179,0.8)"
                                   :borderColor "#ffdab3"
                                   :borderWidth 2
                                   :borderRadius 4
                                   :borderSkipped false}]}
                      (assoc-in chart-defaults [:scales :y :ticks :stepSize]
                                (if (> (apply max (vals cats)) 0)
                                  (/ (apply max (vals cats)) 5)
                                  10))))))
    :reagent-render
    (fn [expenses]
      (if (seq expenses)
        [:div.chart-container [:canvas#category-chart]]
        [:div.empty-state
         [:div.empty-state-title "No expenses"]
         [:div.empty-state-text "Add expenses in CLI version"]]))}))
;; @secend->@secname <categorychart>

;; @secstart->@secname <dailychart>
  ;; @funcinfo <line chart of spending by date, renders empty state if no expenses>
(defn daily-chart [expenses]
  (r/create-class
   {:component-did-mount
    (fn []
      (when (seq expenses)
        (let [daily  (by-date expenses)
              sorted (sort (keys daily))]
          (make-chart "daily-chart" "line"
                      {:labels (clj->js sorted)
                       :datasets [{:data (clj->js (map daily sorted))
                                   :borderColor "#c8aaaa"
                                   :backgroundColor "rgba(200,170,170,0.1)"
                                   :tension 0.3
                                   :fill true
                                   :pointBackgroundColor "#c8aaaa"
                                   :pointBorderColor "#2a1f33"
                                   :pointBorderWidth 2
                                   :pointRadius 4
                                   :borderWidth 3}]}
                      chart-defaults))))
    :reagent-render
    (fn [expenses]
      (if (seq expenses)
        [:div.chart-container [:canvas#daily-chart]]
        [:div.empty-state
         [:div.empty-state-title "No expenses"]
         [:div.empty-state-text "Add expenses in CLI version"]]))}))
;; @secend->@secname <dailychart>

;; @secstart->@secname <monthlychart>
  ;; @funcinfo <bar chart of total spending per month, renders empty state if no expenses>
(defn monthly-chart [expenses]
  (r/create-class
   {:component-did-mount
    (fn []
      (when (seq expenses)
        (let [monthly (by-month expenses)
              sorted  (sort (keys monthly))]
          (make-chart "monthly-chart" "bar"
                      {:labels (clj->js sorted)
                       :datasets [{:data (clj->js (map monthly sorted))
                                   :backgroundColor "rgba(159,131,131,0.8)"
                                   :borderColor "#9f8383"
                                   :borderWidth 2
                                   :borderRadius 4
                                   :borderSkipped false}]}
                      (assoc-in chart-defaults [:scales :y :ticks :stepSize]
                                (if (> (apply max (vals monthly)) 0)
                                  (/ (apply max (vals monthly)) 5)
                                  10))))))
    :reagent-render
    (fn [expenses]
      (if (seq expenses)
        [:div.chart-container [:canvas#monthly-chart]]
        [:div.empty-state
         [:div.empty-state-title "No expenses"]
         [:div.empty-state-text "Add expenses in CLI version"]]))}))
;; @secend->@secname <monthlychart>

;; @secstart->@secname <piechart>
  ;; @funcinfo <pie chart of category distribution with custom legend and tooltip>
(defn pie-chart [expenses]
  (r/create-class
   {:component-did-mount
    (fn []
      (when (seq expenses)
        (let [cats          (by-category expenses)
              labels        (clj->js (keys cats))
              data          (clj->js (vals cats))
              legend-config {:display true
                             :position "bottom"
                             :labels {:color "#f5ede4"
                                      :padding 15
                                      :font {:family "Pixelify Sans"
                                             :size 14}}}
              tooltip-config {:callbacks {:label (fn [ctx]
                                                   (let [label (.-label ctx)
                                                         value (.-formattedValue ctx)]
                                                     (str label ": " value " EUR")))}}]
          (make-chart "pie-chart" "pie"
                      {:labels labels
                       :datasets [{:data data
                                   :backgroundColor (clj->js (take (count cats) chart-colors))
                                   :borderColor "#2a1f33"
                                   :borderWidth 3}]}
                      (-> chart-defaults
                          (assoc-in [:plugins :legend] legend-config)
                          (assoc-in [:plugins :tooltip] tooltip-config))))))
    :reagent-render
    (fn [expenses]
      (if (seq expenses)
        [:div.chart-container [:canvas#pie-chart]]
        [:div.empty-state
         [:div.empty-state-title "No expenses"]
         [:div.empty-state-text "Add expenses in CLI version"]]))}))
;; @secend->@secname <piechart>

;; @secstart->@secname <statssummary>
  ;; @funcinfo <renders total record count and total amount spent for all time>
(defn stats-summary [expenses]
  (let [total (reduce + 0 (map :amount expenses))]
    [:section.stats-summary
     [:div.stat-card
      [:span "Total records:"]
      [:span (count expenses)]]
     [:div.stat-card
      [:span "Total spent:"]
      [:span (.toLocaleString total "ru-RU") " EUR"]]]))
;; @secend->@secname <statssummary>

;; @secstart->@secname <statisticspage>
  ;; @funcinfo <statistics page root component with summary and all four charts>
(defn statistics-page []
  (let [expenses @(rf/subscribe [:expenses])]
    [:main.main-container.statistic-page
     [:h1.page-title "Expense Statistics"]
     ;; @info <all time summary stats>
     [stats-summary expenses]
     ;; @secstart->@secname <chartsgrid> :: @secinfo <2x2 grid of all charts>
     [:section.charts-section
      [:div.charts-grid
       [:div.chart-card [:h2 "Spending by category"]  [category-chart expenses]]
       [:div.chart-card [:h2 "Spending by day"]       [daily-chart expenses]]
       [:div.chart-card [:h2 "Spending by month"]     [monthly-chart expenses]]
       [:div.chart-card [:h2 "Category distribution"] [pie-chart expenses]]]]
     ;; @secend->@secname <chartsgrid>
     ]))
;; @secend->@secname <statisticspage>