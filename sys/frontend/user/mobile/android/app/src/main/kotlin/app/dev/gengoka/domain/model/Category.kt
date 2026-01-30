package app.dev.gengoka.domain.model

data class Category(
    val id: String,
    val name: String,
    val description: String?,
    val icon: String?,
    val color: String?,
    val charLimit: Int,
    val sortOrder: Int
) {
    companion object {
        val defaultIcons = mapOf(
            "状況描写" to "location_on",
            "要約力" to "menu_book",
            "感性の言語化" to "mood",
            "言い換え" to "sync_alt",
            "概念説明" to "lightbulb"
        )

        val defaultColors = mapOf(
            "状況描写" to "#FFF3E0",
            "要約力" to "#E3F2FD",
            "感性の言語化" to "#FCE4EC",
            "言い換え" to "#E8F5E9",
            "概念説明" to "#F3E5F5"
        )

        val defaultDescriptions = mapOf(
            "状況描写" to "その場にいない人に伝える",
            "要約力" to "ストーリーを短く伝える",
            "感性の言語化" to "気持ちを言葉にする",
            "言い換え" to "別の表現で伝える",
            "概念説明" to "わかりやすく例える"
        )
    }

    fun getDisplayIcon(): String = icon ?: defaultIcons[name] ?: "quiz"
    fun getDisplayColor(): String = color ?: defaultColors[name] ?: "#F5F5F5"
    fun getDisplayDescription(): String = description ?: defaultDescriptions[name] ?: ""
}
