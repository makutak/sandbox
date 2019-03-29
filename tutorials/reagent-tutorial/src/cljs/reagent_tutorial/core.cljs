(ns reagent-tutorial.core
  (:require
   [reagent.core :as reagent]))

(def data [{:author "Pete Hunt", :text "This is one comment"}
           {:author "Jordan Walke", :text "This is *another* comment"}])

(defn comment-item
  [author & children]
  (into [:div.comment
         [:h2.commentAuthor author]]
        children))

(defn comment-list
  [data]
  [:div.commentList
   (for [comment data]
     [comment-item (:author comment) (:text comment)])])

(defn comment-form
  []
  [:div.commentForm
   "Hello, world! I am a CommentForm"])


(defn comment-box
  []
  [:div.commentBox
   [comment-list data]])

;; -------------------------
;; Initialize app

(defn mount-root []
  (reagent/render [comment-box] (.getElementById js/document "app")))

(defn init! []
  (mount-root))
